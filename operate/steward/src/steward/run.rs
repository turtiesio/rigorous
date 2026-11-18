//! One day: undertake that each claim holds, run what must be run, record what was found.

use super::*;

impl Steward {
    /// One pass: observe the files, undertake today's upkeep, run only the checks whose claims
    /// are not currently evidenced, and persist the log.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault. A failing check is not an error: it is a finding.
    pub fn run(&mut self, day: u64, force: bool, skip: &BTreeSet<Check>) -> Result<Vec<(Check, Outcome)>, StewardError> {
        let _ = self.world.tick(day)?;
        // Undertake any roadmap item not yet undertaken (whatever its status now).
        let undertaken: BTreeSet<Prop> = self
            .world
            .tagged(Tag::Task)?
            .into_iter()
            .filter_map(|r| match &r.terms().cons {
                Condition::Holds(p) => Some(p.clone()),
                Condition::Unrefuted(_) | Condition::Paid { .. } => None,
            })
            .collect();
        for item in &BACKLOG {
            let prop = Prop::Conforms(item.requirement.clone());
            if !undertaken.contains(&prop) {
                let _ = self.world.act(&MAINTAINER, Act::Commit(undertaking(prop, Time(day).plus(item.review_within), Tag::Task)))?;
            }
        }
        let _ = self.watcher.scan(&mut self.world)?;
        let mut outcomes = Vec::new();
        for check in Check::ALL {
            // Two propositions. "It conforms" is about the files as they are now: its evidence is
            // bound to their digests and goes stale when they change. "It held on day d" is a
            // fact about the past: nothing that happens later can make it stale, so a good day
            // stays good and a bad day stays bad whatever is done to the files afterwards.
            let now_prop = check.prop();
            let day_prop = Prop::HeldOn { requirement: check.requirement(), day };
            // A duplicate (a second run on the same day) is refused by the norms unit; that is fine.
            let _ = self.world.act(&MAINTAINER, Act::Commit(undertaking(day_prop.clone(), Time(day), Tag::Upkeep)))?;
            let outcome = if skip.contains(&check) {
                Outcome::Skipped
            } else if !force && self.evidenced(check)? {
                Outcome::StillEvidenced
            } else {
                match run_check(&mut self.world, &mut self.watcher, &mut check.command(&self.root), &now_prop, &[check.artifact()], &CI) {
                    Ok(true) => Outcome::RanAndPassed,
                    Ok(false) => Outcome::RanAndFailed,
                    Err(AdapterError::Io(_)) => Outcome::CouldNotObserve,
                    Err(e) => return Err(e.into()),
                }
            };
            // The day's fact: what a real check, on the files as they are now, shows as of this run.
            if !matches!(outcome, Outcome::Skipped | Outcome::CouldNotObserve) {
                use rigor_institution::model::About;
                let holds = self.evidenced(check)?;
                let recorded = self.world.epi()?.assessment(&day_prop);
                if (recorded == Assessment::Accepted) != holds || recorded == Assessment::Unsupported {
                    let on_it = self.world.epi()?.evidence_on(&day_prop);
                    let mine: Vec<_> = self.world.epi()?.standing_evidence().filter(|e| on_it.contains(&e.id) && e.source == CI).map(|e| e.id).collect();
                    for e in mine {
                        let _ = self.world.act(&CI, Act::Retract { evidence: e })?;
                    }
                    let about = if holds { About::Supports(day_prop) } else { About::Rebuts(day_prop) };
                    let _ = self.world.act(&CI, attest(about, Method::Test, &format!("as of this run on day {day}")))?;
                }
            }
            outcomes.push((check, outcome));
        }
        // A lapse is put right when the requirement that lapsed is evidenced again: say so, as
        // evidence, so that the remediation duty the lapse created can be discharged.
        let mut put_right = Vec::new();
        for r in self.world.tagged(Tag::Remediation)? {
            let lapsed = r.terms().basis.as_ref().and_then(|b| self.world.norms().ok()?.record(&b.of).map(|x| x.terms().cons.clone()));
            let check = match lapsed {
                Some(Condition::Holds(Prop::HeldOn { requirement, .. } | Prop::Conforms(requirement))) => {
                    Check::ALL.into_iter().find(|c| c.requirement() == requirement)
                }
                _ => None,
            };
            if let (Some(check), Condition::Holds(remediated), Some(Status::Detached)) = (check, &r.terms().cons, r.status) {
                if self.evidenced(check)? {
                    put_right.push(remediated.clone());
                }
            }
        }
        for p in put_right {
            let _ = self.world.act(&CI, attest(rigor_institution::model::About::Supports(p), Method::Test, "the lapsed requirement is evidenced again"))?;
        }
        self.save()?;
        Ok(outcomes)
    }
}
