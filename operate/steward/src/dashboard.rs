//! One page that shows what is going on.
//!
//! Built from the log and from the system's examination of itself; it holds no state of its
//! own. The order is the order of urgency for the person responsible: what needs them first,
//! then whether the system's claims about itself hold, then what it can and cannot do, then
//! what it has undertaken, then what it has been doing.

use crate::change::{Missing, Progress};
use crate::selfmodel::{describe_need, SelfReport, Standing};
use crate::{Check, Steward, StewardError, OWNER};
use rigor_institution::model::{Assessment, Condition, Event, Prop, Status, Tag};
use std::fmt::Write as _;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

const STYLE: &str = r"
:root{--ok:var(--success,#1a7f37);--bad:var(--destructive,#cf222e);--warn:var(--warning,#9a6700);--line:var(--border,#d0d7de);--dim:var(--muted-foreground,#57606a)}
body{font-family:var(--font-sans,system-ui,sans-serif);color:var(--foreground,#1f2328);margin:0;font-size:14px;line-height:1.45}
h1{font-size:18px;margin:0 0 2px}h2{font-size:13px;text-transform:uppercase;letter-spacing:.06em;color:var(--dim);margin:22px 0 8px}
.sub{color:var(--dim);margin-bottom:6px}
.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(150px,1fr));gap:8px}
.tile{border:1px solid var(--line);border-radius:var(--radius,8px);padding:10px 12px}
.tile b{display:block;font-size:20px}.tile span{color:var(--dim);font-size:12px}
table{border-collapse:collapse;width:100%}td,th{text-align:left;padding:5px 8px 5px 0;border-bottom:1px solid var(--line);vertical-align:top}
th{font-weight:600;color:var(--dim);font-size:12px}
.ok{color:var(--ok);font-weight:600}.bad{color:var(--bad);font-weight:600}.warn{color:var(--warn);font-weight:600}.dim{color:var(--dim)}
.need{border-left:3px solid var(--warn);padding:6px 10px;margin:6px 0}
code{font-family:var(--font-mono,ui-monospace,monospace);font-size:12px}
";

fn status_class(s: Status) -> &'static str {
    match s {
        Status::Satisfied => "ok",
        Status::Violated => "bad",
        Status::Detached | Status::Conditional => "warn",
        Status::Expired | Status::Released | Status::Cancelled | Status::Void => "dim",
    }
}

impl Steward {
    /// Render the dashboard as one self-contained HTML page.
    ///
    /// # Errors
    /// If the world lacks an institutional unit, or a change request cannot be read.
    pub fn dashboard(&self, day: u64, about_itself: &SelfReport) -> Result<String, StewardError> {
        let epi = self.world.epi()?;
        let mut h = String::new();
        let _ = write!(h, "<!doctype html><html><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>rigor: what is going on</title><style>{STYLE}</style></head><body>");
        let _ = write!(
            h,
            "<h1>What is going on</h1><div class=\"sub\">day {day} &middot; {} events on record &middot; record intact: {}</div>",
            self.world.rt.log().next_seq(),
            if self.world.rt.log().verify().is_ok() { "yes" } else { "NO" }
        );

        // 1. What needs the owner.
        let ids = self.change_ids()?;
        let changes: Vec<Progress> = ids.iter().filter_map(|id| self.progress(id).ok()).collect();
        let unreadable: Vec<_> = ids.iter().filter(|id| self.progress(id).is_err()).collect();
        let mut asks: Vec<String> = Vec::new();
        for id in &unreadable {
            asks.push(format!("The stored change request <code>{id}</code> does not match the identity it was recorded under (altered, or written by an older version). It cannot be verified or applied."));
        }
        for c in changes.iter().filter(|c| !c.applied && c.missing == vec![Missing::OwnersApproval]) {
            asks.push(format!(
                "Approve or reject the change <b>{}</b> (<code>{}</code>): it is verified and waits only for you. It adds {} file(s), edits {}; {} line(s) removed, {} new.",
                esc(&c.title),
                c.id,
                c.size.files_added,
                c.size.files_edited,
                c.size.lines_removed,
                c.size.lines_new
            ));
        }
        for c in changes.iter().filter(|c| {
            !c.applied
                && c.missing.contains(&Missing::OwnersApproval)
                && c.missing.contains(&Missing::AssessedSound)
                && !c.missing.contains(&Missing::PassingVerification)
        }) {
            asks.push(format!("The change <b>{}</b> (<code>{}</code>) passed verification but has a standing objection. Rule on it.", esc(&c.title), c.id));
        }
        if self.budget_left()? < 30_000 {
            asks.push("The budget for consulting models is spent or was never set. Research and review are stopped until you grant one.".to_owned());
        }
        let overdue: Vec<_> =
            self.commitments(Tag::Task)?.into_iter().filter(|(c, s)| *s == Status::Violated || (*s == Status::Detached && c.terms.deadline.0 < day)).collect();
        for (c, _) in &overdue {
            asks.push(format!("A roadmap undertaking is past its review day: <b>{}</b>.", esc(&cond(&c.terms.cons))));
        }
        let _ = write!(h, "<h2>Needs you</h2>");
        if asks.is_empty() {
            let _ = write!(h, "<div class=\"dim\">Nothing is waiting for the owner, as of day {day}.</div>");
        }
        for a in &asks {
            let _ = write!(h, "<div class=\"need\">{a}</div>");
        }

        // 2. Summary tiles.
        let (limit, spent) = self.budgets()?.iter().fold((0u64, 0u64), |(l, s), (_, lim, sp)| (l + lim, s + sp));
        let holds = Check::ALL.iter().filter(|c| self.evidenced(**c).unwrap_or(false)).count();
        let can = about_itself.abilities.iter().filter(|(_, s)| !matches!(s, Standing::Cannot(_))).count();
        let questions = crate::research::agenda();
        let answered = questions.iter().filter(|q| epi.assessment(&crate::research::answered(&q.id)) == Assessment::Accepted).count();
        let _ = write!(h, "<h2>At a glance</h2><div class=\"grid\">");
        for (value, label) in [
            (format!("{holds} / {}", Check::ALL.len()), "claims about itself currently evidenced"),
            (format!("{can} / {}", about_itself.abilities.len()), "abilities it has now"),
            (format!("{answered} / {}", questions.len()), "research questions with an accepted answer"),
            (format!("{} / {}", changes.iter().filter(|c| c.applied).count(), changes.len()), "change requests applied"),
            (format!("${:.4} / ${:.2}", micro(spent), micro(limit)), "spent of the model budget"),
        ] {
            let _ = write!(h, "<div class=\"tile\"><b>{value}</b><span>{label}</span></div>");
        }
        let _ = write!(h, "</div>");

        // 3. Its claims about itself: what was checked, by what means, and when. "Checks passed"
        //    is never shown as "this is correct", and no evidence looks different from evidence against.
        let _ = write!(h, "<h2>What it claims about itself</h2><table><tr><th>Claim</th><th>Standing</th><th>By what means</th><th>Evidence from</th></tr>");
        for c in Check::ALL {
            let standing = epi.standing(&c.prop());
            let (class, word) = wording(standing.assessment);
            let means: Vec<String> = epi.evidence().filter(|e| standing.basis.contains(&e.id)).map(|e| format!("{:?} by {}", e.method, e.source)).collect();
            let from = standing
                .since
                .map_or_else(|| "&ndash;".to_owned(), |t| if t.0 == day { "today".to_owned() } else { format!("{} days ago", day.saturating_sub(t.0)) });
            let _ = write!(
                h,
                "<tr><td>{}</td><td class=\"{class}\">{word}</td><td>{}</td><td>{from}</td></tr>",
                c.requirement(),
                if means.is_empty() { "&ndash;".to_owned() } else { means.join(", ") }
            );
        }
        let _ = write!(h, "</table><div class=\"sub\">A real check that passed is evidence that the check passed, not that the thing is correct.</div>");

        // 3a. Work stalled by one of its own limits.
        let _ = write!(h, "<h2>Stalled by its own limits</h2>");
        let open_questions = questions.len() - answered;
        let mut stalled = Vec::new();
        if open_questions > 0 {
            if let Some(Standing::Cannot(lacking)) = about_itself.standing(crate::selfmodel::Ability::Research) {
                stalled.push(format!(
                    "{open_questions} research question(s) are open and it cannot research: lacking {}",
                    lacking.iter().map(|n| esc(&describe_need(n))).collect::<Vec<_>>().join("; ")
                ));
            }
        }
        if changes.iter().any(|c| !c.applied && c.missing.contains(&Missing::PassingVerification)) {
            if let Some(Standing::Cannot(lacking)) = about_itself.standing(crate::selfmodel::Ability::VerifyChanges) {
                stalled.push(format!(
                    "change request(s) await verification and it cannot verify: lacking {}",
                    lacking.iter().map(|n| esc(&describe_need(n))).collect::<Vec<_>>().join("; ")
                ));
            }
        }
        if stalled.is_empty() {
            let _ = write!(h, "<div class=\"dim\">No work is stalled by something it cannot do.</div>");
        }
        for s in &stalled {
            let _ = write!(h, "<div class=\"need\">{s}</div>");
        }

        // 4. What it can and cannot do.
        let _ =
            write!(h, "<h2>What it can do, what it cannot, and what it would need</h2><table><tr><th>Ability</th><th>Now</th><th>What is lacking</th></tr>");
        for (a, s) in &about_itself.abilities {
            let (class, word, lacking) = match s {
                Standing::Can => ("ok", "can", String::new()),
                Standing::CanWithApproval => ("warn", "can, with your approval each time", String::new()),
                Standing::Cannot(needs) => ("bad", "cannot", needs.iter().map(|n| esc(&describe_need(n))).collect::<Vec<_>>().join("<br>")),
            };
            let _ = write!(h, "<tr><td>{}<div class=\"dim\">{}</div></td><td class=\"{class}\">{word}</td><td>{lacking}</td></tr>", a.name(), esc(a.meaning()));
        }
        let _ = write!(h, "</table>");

        // 5. Changes.
        let _ = write!(h, "<h2>Changes: requested, verified, approved, applied</h2>");
        if changes.is_empty() {
            let _ = write!(h, "<div class=\"dim\">No change has been requested.</div>");
        } else {
            let _ = write!(h, "<table><tr><th>Change</th><th>By</th><th>Assessed</th><th>Still missing</th></tr>");
            for c in &changes {
                let missing = if c.applied {
                    "<span class=\"ok\">applied</span>".to_owned()
                } else {
                    c.missing.iter().map(|m| format!("{m:?}")).collect::<Vec<_>>().join(", ")
                };
                let _ = write!(
                    h,
                    "<tr><td>{}<div class=\"dim\"><code>{}</code></div></td><td>{}</td><td>{:?}</td><td>{missing}</td></tr>",
                    esc(&c.title),
                    c.id,
                    c.proposed_by,
                    c.assessment
                );
            }
            let _ = write!(h, "</table>");
        }

        // 6. Research.
        let _ = write!(h, "<h2>Research agenda</h2><table><tr><th>Question</th><th>State</th></tr>");
        for q in &questions {
            let state = epi.assessment(&crate::research::answered(&q.id));
            let (class, word) =
                if state == Assessment::Accepted { ("ok", "answered; accepted on a second model's review (weaker than a check)") } else { ("dim", "open") };
            let _ = write!(
                h,
                "<tr><td>{}<div class=\"dim\">{}</div></td><td class=\"{class}\">{word}</td></tr>",
                q.id,
                esc(&q.text.chars().take(150).collect::<String>())
            );
        }
        let _ = write!(h, "</table>");

        // 7. Undertakings.
        let _ = write!(h, "<h2>What it has undertaken</h2><table><tr><th>Undertaking</th><th>Owed by</th><th>Review day</th><th>Status</th></tr>");
        let mut tasks = self.commitments(Tag::Task)?;
        tasks.sort_by_key(|(c, _)| c.terms.deadline);
        for (c, s) in &tasks {
            let when =
                if c.terms.deadline.0 >= day { format!("in {} days", c.terms.deadline.0 - day) } else { format!("{} days ago", day - c.terms.deadline.0) };
            let _ = write!(
                h,
                "<tr><td>{}</td><td>{}</td><td>{when}</td><td class=\"{}\">{s:?}</td></tr>",
                esc(&cond(&c.terms.cons)),
                c.terms.debtor,
                status_class(*s)
            );
        }
        let _ = write!(h, "</table>");
        let upkeep = self.commitments(Tag::Upkeep)?;
        let count = |s: Status| upkeep.iter().filter(|(_, x)| *x == s).count();
        let _ = write!(
            h,
            "<div class=\"sub\" style=\"margin-top:8px\">Daily upkeep on record: {} undertakings, {} held, <span class=\"{}\">{} lapsed</span>.</div>",
            upkeep.len(),
            count(Status::Satisfied),
            if count(Status::Violated) > 0 { "bad" } else { "dim" },
            count(Status::Violated)
        );

        // 8. Recent activity.
        let _ = write!(h, "<h2>Most recent acts</h2><table><tr><th>#</th><th>Who</th><th>On whose authority</th><th>What</th></tr>");
        let recent: Vec<_> = self
            .world
            .rt
            .log()
            .entries()
            .iter()
            .rev()
            .filter(|e| matches!(e.event, Event::Ok { .. } | Event::Denied { .. } | Event::Delegated(_)))
            .take(14)
            .collect();
        for e in recent {
            let (who, what) = match &e.event {
                Event::Ok { by, act, .. } => (by.to_string(), format!("{:?}", act.kind())),
                Event::Denied { by, act, reason, .. } => (by.to_string(), format!("<span class=\"bad\">refused</span> {:?}: {reason:?}", act.kind())),
                Event::Delegated(c) => (OWNER.to_string(), format!("delegated to {}", c.holder)),
                _ => continue,
            };
            let authority = if who == OWNER.to_string() { "<b>the owner's decision</b>" } else { "its own" };
            let _ = write!(h, "<tr><td class=\"dim\">{}</td><td>{who}</td><td>{authority}</td><td>{what}</td></tr>", e.seq);
        }
        let _ = write!(h, "</table></body></html>");
        Ok(h)
    }
}

/// How a standing is worded: no evidence must not look like evidence against.
const fn wording(a: Assessment) -> (&'static str, &'static str) {
    match a {
        Assessment::Accepted => ("ok", "evidenced"),
        Assessment::Refuted => ("bad", "evidence against"),
        Assessment::Contested => ("warn", "evidence both ways"),
        Assessment::Unsupported => ("warn", "no current evidence"),
    }
}

fn micro(n: u64) -> f64 {
    f64::from(u32::try_from(n).unwrap_or(u32::MAX)) / 1_000_000.0
}

fn cond(c: &Condition) -> String {
    match c {
        Condition::Holds(Prop::Conforms(r)) | Condition::Unrefuted(Prop::Conforms(r)) => r.to_string(),
        Condition::Holds(Prop::HeldOn { requirement, day }) => format!("{requirement} on day {day}"),
        Condition::Holds(p) | Condition::Unrefuted(p) => p.to_string(),
        Condition::Paid { at_least, .. } => format!("payment of at least {}", at_least.0),
    }
}
