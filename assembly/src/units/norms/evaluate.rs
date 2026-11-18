//! The status of a commitment as a function of the facts: when its conditions came to hold, and what that makes it.

use super::*;

impl Norms {
    pub(super) fn when(&self, r: &Record, cond: Option<&Condition>, occurred: Option<Time>) -> Option<Time> {
        match cond {
            None => Some(r.created_at),
            Some(Condition::Holds(p)) => match self.prop.get(p) {
                Some((Assessment::Accepted, since, _)) => *since,
                Some(_) | None => None,
            },
            // A maintenance condition is never "brought about": see `evaluate`.
            Some(Condition::Unrefuted(_)) => None,
            Some(Condition::Paid { .. }) => occurred,
        }
    }

    /// The status function. First matching line wins.
    #[must_use]
    pub fn evaluate(&self, r: &Record) -> Status {
        let t = r.terms();
        match r.op {
            Some(Op::Released) => return Status::Released,
            Some(Op::Cancelled) => return Status::Cancelled,
            Some(Op::Void) => return Status::Void,
            // Walking away is final: what others do afterwards does not undo it.
            Some(Op::Breach) => return Status::Violated,
            None => {}
        }
        if self.when(r, Some(&t.cons), r.cons_at).is_some_and(|at| at <= t.deadline) {
            return Status::Satisfied;
        }
        let late = self.now > t.deadline;
        if let Condition::Unrefuted(p) = &t.cons {
            // Maintenance: violated while the proposition stands refuted, if the refutation came
            // within the period; discharged when the period ends without one.
            let detached = self.when(r, t.ant.as_ref(), r.ant_at).is_some_and(|at| at <= t.deadline);
            let refuted_in_time = matches!(self.prop.get(p), Some((Assessment::Refuted, _, learned)) if *learned <= t.deadline);
            return match (detached, refuted_in_time, late) {
                (false, _, true) => Status::Expired,
                (false, _, false) => Status::Conditional,
                (true, true, _) => Status::Violated,
                (true, false, true) => Status::Satisfied,
                (true, false, false) => Status::Detached,
            };
        }
        match self.when(r, t.ant.as_ref(), r.ant_at) {
            Some(at) if at <= t.deadline => {
                if late {
                    Status::Violated
                } else {
                    Status::Detached
                }
            }
            Some(_) | None => {
                if late {
                    Status::Expired
                } else {
                    Status::Conditional
                }
            }
        }
    }
}
