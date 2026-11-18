//! Amending the rule set: what can be changed, and the checks a changed rule set must pass.

use super::*;
use rigor_logic::datalog::{self};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// The amendable parts of a rule set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RulePath {
    Ranks,
    Trust,
    SelfSupport,
    Templates,
    Period,
    Responsibility,
    Retroactive,
    Quorum,
    Entrenched,
}

/// One change to the rule set.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Amendment {
    SetRank(Method, Option<Rank>),
    SetTrust(Method, Trust),
    SetSelfSupport(bool),
    AddViolationTemplate(ViolationTemplate),
    AddVerdictTemplate(VerdictTemplate),
    SetMaintenancePeriod(u64),
    AddResponsibilityRules(Vec<RRule>),
    SetRetroactive(bool),
    SetQuorum(u32),
    Entrench(RulePath),
    Disentrench(RulePath),
}

impl Amendment {
    #[must_use]
    pub const fn path(&self) -> RulePath {
        match self {
            Self::SetRank(..) => RulePath::Ranks,
            Self::SetTrust(..) => RulePath::Trust,
            Self::SetSelfSupport(_) => RulePath::SelfSupport,
            Self::AddViolationTemplate(_) | Self::AddVerdictTemplate(_) => RulePath::Templates,
            Self::SetMaintenancePeriod(_) => RulePath::Period,
            Self::AddResponsibilityRules(_) => RulePath::Responsibility,
            Self::SetRetroactive(_) => RulePath::Retroactive,
            Self::SetQuorum(_) => RulePath::Quorum,
            Self::Entrench(_) | Self::Disentrench(_) => RulePath::Entrenched,
        }
    }
}

/// Why a rule set is not acceptable.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RulesError {
    Responsibility(RRuleError),
    QuorumZero,
    PeriodZero,
    /// Obligations could regress forever through this tag.
    TemplateCycle(Tag),
}

impl Rules {
    /// The rule set after applying amendments (not yet validated).
    #[must_use]
    pub fn amended(&self, amendments: &[Amendment]) -> Self {
        let mut r = self.clone();
        for a in amendments {
            match a {
                Amendment::SetRank(m, Some(rank)) => {
                    let _ = r.evidence.ranks.insert(*m, *rank);
                }
                Amendment::SetRank(m, None) => {
                    let _ = r.evidence.ranks.remove(m);
                }
                Amendment::SetTrust(m, t) => {
                    let _ = r.evidence.trusted.insert(*m, t.clone());
                }
                Amendment::SetSelfSupport(b) => r.evidence.self_support = *b,
                Amendment::AddViolationTemplate(t) => r.norms.on_violation.push(t.clone()),
                Amendment::AddVerdictTemplate(t) => r.norms.on_verdict.push(t.clone()),
                Amendment::SetMaintenancePeriod(p) => r.norms.maintenance_period = *p,
                Amendment::AddResponsibilityRules(rs) => r.responsibility.rules.extend(rs.iter().cloned()),
                Amendment::SetRetroactive(b) => r.responsibility.retroactive = *b,
                Amendment::SetQuorum(q) => r.governance.quorum = *q,
                Amendment::Entrench(p) => {
                    let _ = r.governance.entrenched.insert(*p);
                }
                Amendment::Disentrench(p) => {
                    let _ = r.governance.entrenched.remove(p);
                }
            }
        }
        r
    }

    /// Static validation, performed on the constitution and before any amendment is enacted.
    ///
    /// # Errors
    /// See [`RulesError`].
    pub fn validate(&self) -> Result<(), RulesError> {
        let _ = datalog::check(&self.responsibility.rules).map_err(RulesError::Responsibility)?;
        if self.governance.quorum < 1 {
            return Err(RulesError::QuorumZero);
        }
        if self.norms.maintenance_period == 0 {
            return Err(RulesError::PeriodZero);
        }
        let mut edges: BTreeMap<Tag, BTreeSet<Tag>> = BTreeMap::new();
        for t in &self.norms.on_violation {
            edges.entry(t.tag).or_default().extend(t.create.tags.iter().copied());
        }
        for t in &self.norms.on_verdict {
            edges.entry(t.tag).or_default().extend(t.create.tags.iter().copied());
        }
        // A cycle exists iff some tag reaches itself.
        for start in edges.keys() {
            let mut seen: BTreeSet<Tag> = BTreeSet::new();
            let mut stack: Vec<Tag> = edges.get(start).into_iter().flatten().copied().collect();
            while let Some(t) = stack.pop() {
                if t == *start {
                    return Err(RulesError::TemplateCycle(t));
                }
                if seen.insert(t) {
                    stack.extend(edges.get(&t).into_iter().flatten().copied());
                }
            }
        }
        Ok(())
    }
}

/// The powers a member has by being a member.
#[must_use]
pub fn base_actions() -> BTreeSet<ActionKind> {
    use ActionKind::{Assign, Attest, Cancel, Claim, Commit, Release, Request, Retract, Revoke};
    [Claim, Attest, Retract, Commit, Release, Cancel, Assign, Request, Revoke].into_iter().collect()
}
