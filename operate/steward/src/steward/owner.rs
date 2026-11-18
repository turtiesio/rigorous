//! What only the owner does: accept, decide, grant a budget, re-anchor.

use super::*;

impl Steward {
    /// The project accepts that a backlog item is done (or consciously closed).
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn accept(&mut self, requirement: Requirement, day: u64) -> Result<bool, StewardError> {
        // Only an undertaking on the roadmap can be accepted as done. The project's claims about
        // itself cannot: for those the only evidence is a real run of the check.
        let is_roadmap_item = self.world.tagged(Tag::Task)?.iter().any(|r| r.terms().cons == Condition::Holds(Prop::Conforms(requirement.clone())));
        if !is_roadmap_item || Check::ALL.iter().any(|c| c.requirement() == requirement) {
            return Ok(false);
        }
        let _ = self.world.tick(day)?;
        let _ = self.world.act(&MAINTAINER, Act::Claim { prop: Prop::Conforms(requirement.clone()) })?;
        let _ =
            self.world.act(&PROJECT, attest(rigor_institution::model::About::Supports(Prop::Conforms(requirement)), Method::Ack, "accepted by the project"))?;
        self.save()?;
        Ok(true)
    }

    /// The owner decides a question put to them. Only ever called on the owner's instruction.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn owner_decides(&mut self, requirement: Requirement, yes: bool, day: u64) -> Result<(), StewardError> {
        use rigor_institution::model::About;
        let _ = self.world.tick(day)?;
        let prop = Prop::Conforms(requirement);
        let about = if yes { About::Supports(prop) } else { About::Rebuts(prop) };
        let _ = self.world.act(&OWNER, attest(about, Method::Audit, "decided by the owner"))?;
        self.save()
    }

    /// The owner sets aside a budget for consulting models: a capability to transfer from the
    /// owner's account, limited to `micro_usd`, held by the steward's service account.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn owner_grants_budget(&mut self, micro_usd: u64) -> Result<bool, StewardError> {
        use rigor_institution::model::{ActionKind, Amount, CapId, Grant, Scope};
        let grant = Grant {
            to: CI,
            actions: std::iter::once(ActionKind::Transfer).collect(),
            scope: Scope { from: Some(OWNER), purpose: None },
            limits: std::iter::once((MICRO_USD, Amount(micro_usd))).collect(),
            expires: None,
            delegable: false,
        };
        let entries = self.world.act_with(&OWNER, &CapId::Own(OWNER), Act::Delegate(grant))?;
        self.save()?;
        Ok(entries.iter().any(|e| matches!(e.event, Event::Delegated(_))))
    }

    /// The owner accepts the record as it now is, after an anchor failure has been looked into.
    ///
    /// # Errors
    /// On I/O failure.
    pub fn owner_reanchors(&mut self) -> Result<(), StewardError> {
        self.anchor_problem = None;
        self.save()
    }
}
