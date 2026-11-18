//! Where change requests are kept, and reading them back.

use super::*;

impl Steward {
    pub(super) fn changes_dir(&self) -> PathBuf {
        self.home().join("changes")
    }

    /// The stored request with this identity.
    ///
    /// # Errors
    /// If it cannot be read.
    pub fn change(&self, id: &ChangeId) -> Result<ChangeRequest, StewardError> {
        let text = std::fs::read_to_string(self.changes_dir().join(format!("{id}.json"))).map_err(AdapterError::Io)?;
        let request: ChangeRequest =
            serde_json::from_str(&text).map_err(|e| StewardError::Adapter(AdapterError::Snapshot(rigor_waist::EncodingError(e.to_string()))))?;
        // What was verified and approved is identified by its content. A stored request that
        // no longer matches its identity has been altered since, and is not that request.
        if identity(&request)? != *id {
            return Err(StewardError::ChangeAltered(id.clone()));
        }
        Ok(request)
    }

    /// Every change request on file.
    ///
    /// # Errors
    /// On I/O failure.
    pub fn change_ids(&self) -> Result<Vec<ChangeId>, StewardError> {
        let mut out = Vec::new();
        if let Ok(dir) = std::fs::read_dir(self.changes_dir()) {
            for entry in dir {
                let path = entry.map_err(AdapterError::Io)?.path();
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()).filter(|_| path.extension().is_some_and(|e| e == "json")) {
                    out.push(ChangeId(Name::new(stem)?));
                }
            }
        }
        out.sort();
        Ok(out)
    }
}
