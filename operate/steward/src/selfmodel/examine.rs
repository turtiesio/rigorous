//! Examining itself: probing each need for real and recording the account as evidence.

use super::*;

impl Steward {
    pub(super) fn met(&self, need: &Need, gateway: &Gateway, gateway_answers: bool, depth: u8) -> Result<bool, StewardError> {
        Ok(match need {
            Need::Program(p) => p.present(),
            Need::Credential => gateway.has_credential(),
            Need::GatewayAnswers => gateway_answers,
            Need::Budget => self.budget_left()? > 0,
            Need::OwnerSaysYes(r) => self.world.epi()?.assessment(&Prop::Conforms(r.clone())) == Assessment::Accepted,
            Need::OwnerApprovesEach => true,
            Need::Ability(a) if depth < 8 => {
                let mut all = true;
                for n in a.needs() {
                    all = all && self.met(&n, gateway, gateway_answers, depth + 1)?;
                }
                all
            }
            Need::Ability(_) | Need::Unbuilt(_) | Need::ByDesignNever => false,
        })
    }

    /// Probe every need of every ability, and record what changed as evidence.
    ///
    /// # Errors
    /// On a kernel fault.
    pub fn examine_itself(&mut self, gateway: &Gateway) -> Result<SelfReport, StewardError> {
        let gateway_answers = gateway.has_credential() && gateway.reachable().is_ok();
        let mut abilities = Vec::new();
        for a in Ability::ALL {
            let mut lacking = Vec::new();
            for n in a.needs() {
                if !self.met(&n, gateway, gateway_answers, 0)? {
                    lacking.push(n);
                }
            }
            let standing = if !lacking.is_empty() {
                Standing::Cannot(lacking)
            } else if a.needs().contains(&Need::OwnerApprovesEach) {
                Standing::CanWithApproval
            } else {
                Standing::Can
            };
            // Record only a change: withdraw the probe that no longer describes the facts, attest the new one.
            let prop = Prop::Can(a.name());
            let can = !matches!(standing, Standing::Cannot(_));
            let recorded = self.world.epi()?.standing(&prop);
            let was = match recorded.assessment {
                Assessment::Accepted => Some(true),
                Assessment::Refuted => Some(false),
                Assessment::Contested | Assessment::Unsupported => None,
            };
            if was != Some(can) {
                for old in recorded.basis {
                    let _ = self.world.act(&CI, Act::Retract { evidence: old })?;
                }
                let about = if can { About::Supports(prop) } else { About::Rebuts(prop) };
                let _ = self.world.act(&CI, attest(about, Method::Probe, "probed every need of this ability"))?;
            }
            abilities.push((a, standing));
        }
        Ok(SelfReport { abilities })
    }
}
