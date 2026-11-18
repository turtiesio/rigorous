//! The end-to-end scenario, asserted step by step, plus its variants.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use rigor::actors::CREDIT;
use rigor::model::{
    About, Act, Amendment, Amount, Assessment, Assumption, AttenuationError, BasisKind, CapId, DenyReason, Event, Finding, GovError, Memo, Method, Pred, Prop,
    RProof, Request, RuleId, RulePath, Status, Tag, Task, Time, Version,
};
use rigor::org::duty_to_verify;
use rigor::scenario::{attest, budget, rules, run, world, Cast, AGENT, ALICE, AUDITOR, DEPLOYER, GOV_ALICE, GOV_AUDITOR, MONITOR, TASK1, VENDOR};
use rigor::waist::Entry;
use std::collections::BTreeSet;

fn findings(xs: &[(Finding, rigor::model::ActorId)]) -> BTreeSet<(Finding, rigor::model::ActorId)> {
    xs.iter().cloned().collect()
}

fn verdicts_on(entries: &[Entry<Event>], c: rigor::model::CommitmentId) -> Vec<&rigor::model::Verdict> {
    entries
        .iter()
        .filter_map(|e| match &e.event {
            Event::Verdict(v) if v.commitment == c => Some(v),
            _ => None,
        })
        .collect()
}

#[test]
fn main_scenario() {
    let (w, o) = run(Cast::default(), true, |_| {}).unwrap();
    let done = Prop::Done(Task::whole(TASK1));
    let deployed = Prop::Done(Task::whole(TASK1).step(rigor::actors::DEPLOY));

    // 1. Before the refutation, on the strength of the claim.
    assert_eq!(o.before[&o.c1], Status::Satisfied);
    assert_eq!(o.before[&o.c2], Status::Satisfied);
    assert_eq!(o.before[&o.c0], Status::Detached); // Alice now owes the payment

    // 2. The refutation revises the claim and the obligations.
    let assessed: Vec<_> = o
        .refutation
        .iter()
        .filter_map(|e| match &e.event {
            Event::Assessed { prop, was, assessment, .. } => Some((prop.clone(), *was, *assessment)),
            _ => None,
        })
        .collect();
    assert!(assessed.contains(&(deployed, Assessment::Accepted, Assessment::Refuted)));
    assert!(assessed.contains(&(done, Assessment::Accepted, Assessment::Unsupported))); // its only support fell with the receipt
    assert_eq!(o.after[&o.c1], Status::Violated);
    assert_eq!(o.after[&o.c2], Status::Violated);
    assert_eq!(o.after[&o.c0], Status::Conditional); // the duty to pay no longer exists

    // 3. Secondary obligations are created and tracked to their basis.
    let (audit, rem, refund) = (w.one(&AUDITOR, Tag::Audit).unwrap(), w.one(&VENDOR, Tag::Remediation).unwrap(), w.one(&DEPLOYER, Tag::Refund).unwrap());
    assert_eq!(audit.terms().basis.as_ref().unwrap().of, o.c1);
    assert!(matches!(&rem.terms().basis.as_ref().unwrap().kind, BasisKind::Verdict { finding: Finding::Liable, party, .. } if *party == VENDOR));
    assert_eq!((rem.terms().creditor.clone(), refund.terms().creditor.clone()), (ALICE, ALICE));
    assert_eq!([audit.status, rem.status, refund.status], [Some(Status::Satisfied); 3]);
    assert_eq!(w.one(&ALICE, Tag::Payment).unwrap().status, Some(Status::Expired));

    // 4. Authority and resources.
    assert_eq!(o.over_delegation, DenyReason::NotAnAttenuation(AttenuationError::LimitExceedsParent(CREDIT)));
    assert_eq!(o.overspend, DenyReason::BudgetExceeded(o.cap.clone()));
    assert_eq!(o.revoked_use, DenyReason::Revoked(o.cap.clone()));
    assert_eq!(o.revoked_sub_use, DenyReason::Revoked(o.cap.clone())); // cascade through the chain
    assert!(w.inst().unwrap().audit().is_empty());
    let ledger = w.ledger().unwrap();
    assert!(ledger.conserved());
    assert_eq!([&ALICE, &DEPLOYER, &AGENT].map(|a| ledger.balance(a, &CREDIT)), [Amount(100), Amount(40), Amount(0)]);
    let moved: Vec<_> =
        w.rt.log()
            .entries()
            .iter()
            .filter_map(|e| match &e.event {
                Event::Transferred { transfer: t, .. } => Some((t.from.clone(), t.to.clone(), t.amount, t.memo.clone())),
                _ => None,
            })
            .collect();
    assert_eq!(moved.len(), 2);
    assert_eq!((&moved[0].0, &moved[0].1, moved[0].2), (&ALICE, &DEPLOYER, Amount(30)));
    assert_eq!((&moved[1].0, &moved[1].1, moved[1].2, &moved[1].3), (&DEPLOYER, &ALICE, Amount(30), &Memo::Refund(refund.commitment.id)));

    // 5. Responsibility, with proof and assumptions.
    let adj = w.adj().unwrap();
    assert_eq!(
        adj.findings(&o.c1),
        findings(&[(Finding::Accountable, AGENT), (Finding::Contributed, DEPLOYER), (Finding::Culpable, AGENT), (Finding::Liable, VENDOR)])
    );
    assert_eq!(adj.findings(&o.c2), findings(&[(Finding::Accountable, DEPLOYER), (Finding::Liable, DEPLOYER)]));
    let v = verdicts_on(&o.refutation, o.c1);
    assert_eq!(v.len(), 1, "one verdict per cascade: judged at quiescence only");
    assert_eq!(v[0].rules, Version(1));
    // The AI's liability lifts to its principal by rule liab1.
    assert!(v[0].proofs.iter().any(|p| matches!(p, RProof::Derived { fact, rule, .. } if fact.pred == Pred::Liable && *rule == RuleId::lit("liab1"))));
    assert!(v[0].assumptions.contains(&Assumption::RecordsNotKnowledge));

    // 6. Governance is versioned and guarded.
    assert_eq!(o.entrenched, Some(GovError::Entrenched(RulePath::SelfSupport)));
    assert_eq!(o.enacted, Some(Version(2)));
    let gov = w.gov().unwrap();
    assert_eq!(gov.history().iter().map(|h| h.version).collect::<Vec<_>>(), vec![Version(1), Version(2)]);
    assert_eq!(gov.history()[1].by, [ALICE, AUDITOR].into());
    let enacted: Vec<_> =
        w.rt.log()
            .entries()
            .iter()
            .filter_map(|e| match &e.event {
                Event::RulesChanged(rc) => Some((rc.prev, rc.digest)),
                _ => None,
            })
            .collect();
    assert_eq!(enacted[1].0, Some(enacted[0].1)); // each version names its predecessor

    // 7. Every institutional act was authorised: each Ok is caused by a Do of the same actor.
    let log = w.rt.log().entries();
    for e in log {
        if let Event::Ok { by, act, .. } = &e.event {
            let cause = log.iter().find(|c| Some(c.id) == e.cause).unwrap();
            assert!(matches!(&cause.event, Event::Do { act: a, .. } if a == act));
            assert_eq!(cause.actor.only(), Some(&by.0));
        }
    }
}

#[test]
fn honest_agent_is_excused_and_its_liability_is_withdrawn() {
    // The agent reports the failure. At t=6 its own commitment lapses and, with the service's
    // receipt still standing, the rules hold the vendor liable. When the probe then shows that
    // the service failed, that finding is withdrawn and the remediation duty it created is voided.
    let (w, o) = run(Cast { honest_agent: true, ..Cast::default() }, false, |_| {}).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&Prop::Done(Task::whole(TASK1))), Assessment::Unsupported);
    assert_eq!(w.adj().unwrap().findings(&o.c1), findings(&[(Finding::Accountable, AGENT), (Finding::Contributed, DEPLOYER), (Finding::Excused, AGENT)]));
    assert_eq!(w.one(&VENDOR, Tag::Remediation).unwrap().status, Some(Status::Void));
    assert_eq!(w.one(&DEPLOYER, Tag::Refund).unwrap().status, Some(Status::Satisfied));
    let seq = verdicts_on(w.rt.log().entries(), o.c1);
    assert!(seq.first().unwrap().findings.contains(&(Finding::Liable, VENDOR)));
    assert!(!seq.last().unwrap().findings.contains(&(Finding::Liable, VENDOR)));
}

#[test]
fn rule_change_closes_a_responsibility_gap() {
    // The service lies (says done). Under v1 nobody is at fault for the agent's false report;
    // the v2 amendment (a duty to verify) makes it negligent, and re-adjudication follows.
    let (w, o) = run(Cast { service_truthful: false, ..Cast::default() }, true, |_| {}).unwrap();
    let vs = verdicts_on(w.rt.log().entries(), o.c1);
    assert_eq!(vs.iter().map(|v| v.rules).collect::<Vec<_>>(), vec![Version(1), Version(2)]);
    assert!(!vs[0].findings.contains(&(Finding::Liable, VENDOR)));
    assert!(vs[1].findings.contains(&(Finding::Negligent, AGENT)));
    assert!(vs[1].findings.contains(&(Finding::Liable, VENDOR)));
    // The new finding rests on there being NO record that the agent knew: said so explicitly.
    assert!(vs[1].assumptions.iter().any(|a| matches!(a, Assumption::NoRecordOf(f) if f.pred == Pred::Knew)));
    let rem = w.one(&VENDOR, Tag::Remediation).unwrap();
    assert_eq!((rem.commitment.rules, rem.created_at), (Version(2), Time(7)));
}

#[test]
fn non_retroactive_rules_do_not_reach_back() {
    let mut r = rules();
    r.responsibility.retroactive = false;
    let mut w = world(Cast { service_truthful: false, ..Cast::default() }, r).unwrap();
    let _ = w.act_with(&ALICE, &CapId::Own(ALICE), budget(AGENT, ALICE, 50, true)).unwrap();
    let _ = w.act(&ALICE, Act::Request(Request { to: AGENT, task: Task::whole(TASK1), deadline: Time(5), parent: None, terms: None })).unwrap();
    let _ = w.tick(6).unwrap();
    let deployed = Prop::Done(Task::whole(TASK1).step(rigor::actors::DEPLOY));
    let _ = w.act(&MONITOR, attest(About::Rebuts(deployed), Method::Probe, "x")).unwrap();
    let proposed =
        w.act_with(&ALICE, &CapId::Charter(GOV_ALICE), Act::Propose { amendments: vec![Amendment::AddResponsibilityRules(duty_to_verify())] }).unwrap();
    let id = proposed
        .iter()
        .find_map(|e| match &e.event {
            Event::Proposed { id, .. } => Some(*id),
            _ => None,
        })
        .unwrap();
    let _ = w.act_with(&ALICE, &CapId::Charter(GOV_ALICE), Act::Approve { id }).unwrap();
    let entries = w.act_with(&AUDITOR, &CapId::Charter(GOV_AUDITOR), Act::Approve { id }).unwrap();
    assert_eq!(w.gov().unwrap().version(), Version(2));
    assert!(!entries.iter().any(|e| matches!(e.event, Event::Verdict(_))));
    assert!(w.tagged(Tag::Remediation).unwrap().is_empty());
}

#[test]
fn working_service_everything_is_satisfied() {
    let (w, o) = run(Cast { honest_agent: true, service_works: true, ..Cast::default() }, false, |_| {}).unwrap();
    assert_eq!(w.epi().unwrap().assessment(&Prop::Done(Task::whole(TASK1))), Assessment::Accepted);
    assert_eq!(o.after[&o.c1], Status::Satisfied);
    // The task was done, so Alice's promise to pay 20 detached -- and the script never has her
    // pay. The same rules that judged the agent now judge her: nobody is outside the model.
    assert_eq!(w.one(&ALICE, Tag::Payment).unwrap().status, Some(Status::Violated));
    let all: Vec<_> = w.adj().unwrap().verdicts().map(|(c, f)| (*c, f.clone())).collect();
    assert_eq!(all, vec![(o.c0, findings(&[(Finding::Accountable, ALICE), (Finding::Liable, ALICE)]))]);
}
