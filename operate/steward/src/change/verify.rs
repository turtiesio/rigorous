//! Verifying a change: every check, run on a copy with the change applied.

use super::*;

impl Steward {
    /// Try the change in a sandbox copy of the repository and attest what the project's own
    /// checks say there. Returns whether they all passed.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn verify(&mut self, id: &ChangeId) -> Result<bool, StewardError> {
        let root = self.root().to_path_buf();
        self.verify_with(id, |sandbox| run_checks(&root, sandbox))
    }

    /// As [`Self::verify`], with the checks supplied by the caller (they are given the sandbox
    /// directory). Verification counts only if EVERY one of the project's checks was run:
    /// a check that was not run establishes nothing, so nothing is attested.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn verify_with(&mut self, id: &ChangeId, checks: impl FnOnce(&Path) -> Result<Vec<(Check, bool)>, StewardError>) -> Result<bool, StewardError> {
        let request = self.change(id)?;
        let sandbox = self.home().join("sandbox");
        let _ = std::fs::remove_dir_all(&sandbox);
        std::fs::create_dir_all(&sandbox).map_err(AdapterError::Io)?;
        let l = &crate::layout::BUILD_FILES;
        for entry in l.iter().chain(&crate::layout::CODE).chain([&crate::layout::APP, &crate::layout::PROOFS, &crate::layout::DOCS]) {
            let from = self.root().join(entry);
            if from.exists() {
                let copied = Command::new("cp").arg("-a").arg(&from).arg(&sandbox).status().map_err(AdapterError::Io)?;
                if !copied.success() {
                    return Err(StewardError::Adapter(AdapterError::Io(std::io::Error::other("could not copy the repository into the sandbox"))));
                }
            }
        }
        for (path, content) in request.files.iter().map(|f| (&f.path, &f.content)).chain(request.edits.iter().map(|e| (&e.path, &e.content))) {
            let to = sandbox.join(path);
            if let Some(dir) = to.parent() {
                std::fs::create_dir_all(dir).map_err(AdapterError::Io)?;
            }
            std::fs::write(to, content).map_err(AdapterError::Io)?;
        }
        let results = checks(&sandbox)?;
        let _ = std::fs::remove_dir_all(&sandbox);
        let ran: BTreeSet<Check> = results.iter().map(|(c, _)| *c).collect();
        if Check::ALL.iter().any(|c| !ran.contains(c)) {
            return Ok(false);
        }
        let passed = results.iter().all(|(_, ok)| *ok);
        let summary: Vec<String> = results.iter().map(|(c, ok)| format!("{}={}", c.requirement(), if *ok { "passed" } else { "FAILED" })).collect();
        let prop = Prop::Sound(id.clone());
        let about = if passed { About::Supports(prop) } else { About::Rebuts(prop) };
        let _ = self.world.act(&CI, attest(about, Method::Test, &format!("sandbox verification: {}", summary.join(", "))))?;
        self.persist()?;
        Ok(passed)
    }
}
