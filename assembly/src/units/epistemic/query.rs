//! What can be asked of the evidence unit.

use super::*;

impl Epistemic {
    #[must_use]
    pub fn standing(&self, p: &Prop) -> Standing {
        self.status.get(p).cloned().unwrap_or_default()
    }

    #[must_use]
    pub fn assessment(&self, p: &Prop) -> Assessment {
        self.standing(p).assessment
    }

    pub fn standings(&self) -> impl Iterator<Item = (&Prop, &Standing)> {
        self.status.iter()
    }

    pub fn evidence(&self) -> impl Iterator<Item = &Evidence> {
        self.ev.values().map(|i| &i.ev)
    }

    /// Evidence that has not been withdrawn by its source.
    pub fn standing_evidence(&self) -> impl Iterator<Item = &Evidence> {
        self.ev.values().filter(|i| !i.retracted).map(|i| &i.ev)
    }

    #[must_use]
    pub fn evidence_on(&self, p: &Prop) -> BTreeSet<EvidenceId> {
        self.by_prop.get(p).cloned().unwrap_or_default()
    }

    #[must_use]
    pub fn evidence_about_artifact(&self, a: &ArtifactId) -> BTreeSet<EvidenceId> {
        self.by_art.get(a).cloned().unwrap_or_default()
    }

    /// The proposition every item in the set bears on.
    #[must_use]
    pub fn props_of(&self, ids: &BTreeSet<EvidenceId>) -> BTreeSet<Prop> {
        ids.iter().filter_map(|e| self.ev.get(e)).filter_map(|i| i.prop().cloned()).collect()
    }
}
