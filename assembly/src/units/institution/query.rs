//! What can be asked of the authority unit.

use super::*;

impl Institution {
    #[must_use]
    pub fn spent(&self, cap: &CapId, r: &Resource) -> Amount {
        self.spent.get(&(cap.clone(), r.clone())).copied().unwrap_or_default()
    }

    /// The capability and its ancestors, leaf first, ignoring validity.
    #[must_use]
    pub fn lineage(&self, id: &CapId) -> Vec<&Capability> {
        let mut out = Vec::new();
        let mut cur = self.caps.get(id);
        while let Some(c) = cur {
            out.push(c);
            cur = c.parent.as_ref().and_then(|p| self.caps.get(p));
        }
        out
    }

    /// The chain of a capability if every link is known, unrevoked and unexpired.
    ///
    /// # Errors
    /// The reason the chain is not valid now.
    pub fn chain(&self, id: &CapId) -> Result<Vec<&Capability>, DenyReason> {
        let mut out = Vec::new();
        let mut cur = Some(id.clone());
        while let Some(cid) = cur {
            let c = self.caps.get(&cid).ok_or_else(|| DenyReason::UnknownCap(cid.clone()))?;
            if self.revoked.contains(&cid) {
                return Err(DenyReason::Revoked(cid));
            }
            if c.expires.is_some_and(|e| self.now > e) {
                return Err(DenyReason::Expired(cid));
            }
            out.push(c);
            cur = c.parent.clone();
        }
        Ok(out)
    }

    #[must_use]
    pub fn valid(&self, id: &CapId) -> bool {
        self.chain(id).is_ok()
    }

    /// Invariant check: every delegated capability attenuates its parent and nothing is overspent.
    #[must_use]
    pub fn audit(&self) -> Vec<CapId> {
        let mut bad = Vec::new();
        for c in self.caps.values() {
            let parent_ok = c.parent.as_ref().is_none_or(|p| self.caps.get(p).is_some_and(|p| attenuates(c, p).is_ok()));
            let spend_ok = c.limits.iter().all(|(r, lim)| self.spent(&c.id, r) <= *lim);
            if !(parent_ok && spend_ok) {
                bad.push(c.id.clone());
            }
        }
        bad
    }
}
