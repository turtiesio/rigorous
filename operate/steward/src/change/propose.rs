//! Proposing a change: recorded only if it stays inside the places open to proposals.

use super::*;

impl Steward {
    /// Record a change request. Returns its identity, or why it is not acceptable.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn propose(&mut self, request: &ChangeRequest) -> Result<Result<ChangeId, Refusal>, StewardError> {
        if request.files.is_empty() && request.edits.is_empty() {
            return Ok(Err(Refusal::Empty));
        }
        for f in &request.files {
            if let Err(r) = permitted(self.root(), f) {
                return Ok(Err(r));
            }
        }
        for e in &request.edits {
            if let Err(r) = editable(self.root(), e) {
                return Ok(Err(r));
            }
        }
        let id = identity(request)?;
        std::fs::create_dir_all(self.changes_dir()).map_err(AdapterError::Io)?;
        let text =
            serde_json::to_string_pretty(request).map_err(|e| StewardError::Adapter(AdapterError::Snapshot(rigor_waist::EncodingError(e.to_string()))))?;
        std::fs::write(self.changes_dir().join(format!("{id}.json")), text).map_err(AdapterError::Io)?;
        let _ = self.world.act(&request.proposed_by, Act::Claim { prop: Prop::Sound(id.clone()) })?;
        self.persist()?;
        Ok(Ok(id))
    }
}
