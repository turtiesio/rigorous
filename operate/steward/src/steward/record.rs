//! Keeping and reading the record: saving it, and what it currently shows.

use super::*;

impl Steward {
    /// The repository this steward looks after.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Where the steward keeps what it writes (answers, change requests, the dashboard).
    #[must_use]
    pub fn home(&self) -> PathBuf {
        self.log.parent().map_or_else(|| self.root.join(".rigor"), Path::to_path_buf)
    }

    /// Persist the log.
    ///
    /// # Errors
    /// On I/O failure.
    pub fn persist(&self) -> Result<(), StewardError> {
        self.save()
    }

    pub(super) fn save(&self) -> Result<(), StewardError> {
        if let Some(dir) = self.log.parent() {
            std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
        }
        save(&self.world, &self.log)?;
        // The anchor follows the record -- except when the record has failed to meet it: then
        // it is left as it is, so that the failure cannot be papered over by simply running again.
        if self.anchor_problem.is_none() {
            let anchor = Anchor { events: self.world.rt.log().next_seq(), head: self.world.rt.log().head() };
            let text = serde_json::to_string(&anchor).map_err(|e| StewardError::Adapter(AdapterError::Snapshot(rigor_waist::EncodingError(e.to_string()))))?;
            std::fs::write(self.root.join(".rigor-anchor"), text).map_err(AdapterError::Io)?;
        }
        Ok(())
    }

    /// Whether a self-claim is currently evidenced THE ONLY WAY IT CAN BE: by a real run of its
    /// check, by the service account, bound to the digest of the files as they now are.
    /// Nobody's say-so counts, whatever their standing.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn evidenced(&self, check: Check) -> Result<bool, StewardError> {
        let epi = self.world.epi()?;
        let standing = epi.standing(&check.prop());
        Ok(standing.assessment == Assessment::Accepted
            && epi.standing_evidence().any(|e| standing.basis.contains(&e.id) && e.source == CI && e.method == Method::Test && !e.artifacts.is_empty()))
    }

    /// Whether every self-claim that was not skipped is currently evidenced by a real check,
    /// and the record reaches its anchor.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn conformant(&self, skip: &BTreeSet<Check>) -> Result<bool, StewardError> {
        let mut all = self.anchor_problem.is_none();
        for c in Check::ALL.into_iter().filter(|c| !skip.contains(c)) {
            all = all && self.evidenced(c)?;
        }
        Ok(all)
    }

    /// Commitments carrying a tag, with their current status.
    ///
    /// # Errors
    /// If the world lacks an institutional unit.
    pub fn commitments(&self, tag: Tag) -> Result<Vec<(Commitment, Status)>, StewardError> {
        Ok(self.world.tagged(tag)?.into_iter().filter_map(|r| r.status.map(|s| (r.commitment.clone(), s))).collect())
    }
}
