//! Applying a change: only when verified, approved and assessed sound; exactly what was verified.

use super::*;

impl Steward {
    /// Where a change stands, and what is still missing before it may be applied.
    ///
    /// # Errors
    /// If the request cannot be read or the world lacks a unit.
    pub fn progress(&self, id: &ChangeId) -> Result<Progress, StewardError> {
        let request = self.change(id)?;
        let epi = self.world.epi()?;
        let sound = Prop::Sound(id.clone());
        let standing = epi.standing(&sound);
        let on_it = epi.evidence_on(&sound);
        let evidence: Vec<_> = epi.standing_evidence().filter(|e| on_it.contains(&e.id)).collect();
        let by = |who: &ActorId, method: Method, supports: bool| {
            evidence.iter().any(|e| e.source == *who && e.method == method && matches!(e.about, About::Supports(_)) == supports)
        };
        let applied = epi.assessment(&Prop::Applied(id.clone())) == Assessment::Accepted;
        let mut missing = Vec::new();
        if applied {
            missing.push(Missing::AlreadyApplied);
        }
        if standing.assessment != Assessment::Accepted {
            missing.push(Missing::AssessedSound);
        }
        // A verification that failed is never outweighed: not by a later pass, not by approval.
        if !by(&CI, Method::Test, true) || by(&CI, Method::Test, false) {
            missing.push(Missing::PassingVerification);
        }
        // Nor is a rejection by the owner outweighed by a later approval.
        if !by(&OWNER, Method::Audit, true) || by(&OWNER, Method::Audit, false) {
            missing.push(Missing::OwnersApproval);
        }
        let mut size = Size { files_added: request.files.len(), files_edited: request.edits.len(), ..Size::default() };
        size.lines_new = request.files.iter().map(|f| f.content.lines().count()).sum();
        for e in &request.edits {
            let now = std::fs::read_to_string(self.root().join(&e.path)).unwrap_or_default();
            let (before, kept, new) = line_change(&now, &e.content);
            size.lines_removed += before - kept;
            size.lines_new += new;
        }
        Ok(Progress { id: id.clone(), title: request.title, size, proposed_by: request.proposed_by, assessment: standing.assessment, missing, applied })
    }

    /// Apply a change: write its files into the repository. Refused, with what is missing,
    /// unless it is verified, approved by the owner, and assessed sound.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn apply(&mut self, id: &ChangeId) -> Result<Result<(), Vec<Missing>>, StewardError> {
        let progress = self.progress(id)?;
        if !progress.missing.is_empty() {
            return Ok(Err(progress.missing));
        }
        let request = self.change(id)?;
        // Checked again at the moment of writing: the repository may have changed since the
        // proposal. A change verified against content that is no longer there is not applied.
        if request.files.iter().any(|f| permitted(self.root(), f).is_err()) || request.edits.iter().any(|e| editable(self.root(), e).is_err()) {
            return Ok(Err(vec![Missing::StillFits]));
        }
        for (path, content) in request.files.iter().map(|f| (&f.path, &f.content)).chain(request.edits.iter().map(|e| (&e.path, &e.content))) {
            let to = self.root().join(path);
            if let Some(dir) = to.parent() {
                std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
            }
            std::fs::write(to, content).map_err(AdapterError::Io)?;
        }
        let _ = self.world.act(&CI, attest(About::Supports(Prop::Applied(id.clone())), Method::Probe, "files written to the repository"))?;
        self.persist()?;
        Ok(Ok(()))
    }
}
