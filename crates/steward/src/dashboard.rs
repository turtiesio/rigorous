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
        let changes: Vec<Progress> = self.change_ids()?.iter().filter_map(|id| self.progress(id).ok()).collect();
        let mut asks: Vec<String> = Vec::new();
        for c in changes.iter().filter(|c| !c.applied && c.missing == vec![Missing::OwnersApproval]) {
            asks.push(format!("Approve or reject the change <b>{}</b> (<code>{}</code>): it is verified and waits only for you.", esc(&c.title), c.id));
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
            let _ = write!(h, "<div class=\"dim\">Nothing is waiting for the owner.</div>");
        }
        for a in &asks {
            let _ = write!(h, "<div class=\"need\">{a}</div>");
        }

        // 2. Summary tiles.
        let (limit, spent) = self.budgets()?.iter().fold((0u64, 0u64), |(l, s), (_, lim, sp)| (l + lim, s + sp));
        let holds = Check::ALL.iter().filter(|c| epi.assessment(&c.prop()) == Assessment::Accepted).count();
        let can = about_itself.abilities.iter().filter(|(_, s)| !matches!(s, Standing::Cannot(_))).count();
        let questions = crate::research::agenda();
        let answered =
            questions.iter().filter(|q| crate::research::answered(&q.id).is_ok_and(|r| epi.assessment(&Prop::Conforms(r)) == Assessment::Accepted)).count();
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

        // 3. Its claims about itself.
        let _ = write!(h, "<h2>What it claims about itself, and whether the evidence is current</h2><table><tr><th>Claim</th><th>Assessment</th></tr>");
        for c in Check::ALL {
            let a = epi.assessment(&c.prop());
            let class = if a == Assessment::Accepted { "ok" } else { "bad" };
            let _ = write!(h, "<tr><td>{}</td><td class=\"{class}\">{a:?}</td></tr>", c.requirement());
        }
        let _ = write!(h, "</table>");

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
            let state = crate::research::answered(&q.id).map(|r| epi.assessment(&Prop::Conforms(r))).unwrap_or(Assessment::Unsupported);
            let (class, word) = if state == Assessment::Accepted { ("ok", "answered, reviewed and accepted") } else { ("dim", "open") };
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
        let _ = write!(h, "<h2>Most recent acts</h2><table><tr><th>#</th><th>Who</th><th>What</th></tr>");
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
            let _ = write!(h, "<tr><td class=\"dim\">{}</td><td>{who}</td><td>{what}</td></tr>", e.seq);
        }
        let _ = write!(h, "</table></body></html>");
        Ok(h)
    }
}

fn micro(n: u64) -> f64 {
    f64::from(u32::try_from(n).unwrap_or(u32::MAX)) / 1_000_000.0
}

fn cond(c: &Condition) -> String {
    match c {
        Condition::Holds(Prop::Conforms(r)) | Condition::Unrefuted(Prop::Conforms(r)) => r.to_string(),
        Condition::Holds(p) | Condition::Unrefuted(p) => p.to_string(),
        Condition::Paid { at_least, .. } => format!("payment of at least {}", at_least.0),
    }
}
