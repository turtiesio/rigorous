//! Drafting and reviewing with models, and the owner ruling on a change.

use super::*;

impl Steward {
    /// Have the drafting model revise an existing file to an instruction. Returns the change
    /// request (not yet proposed), or `None` if the budget could not cover the consultation.
    ///
    /// # Errors
    /// On a gateway failure, I/O failure or a kernel fault.
    pub fn draft_edit(&mut self, gateway: &Gateway, path: &str, instruction: &str) -> Result<Option<ChangeRequest>, StewardError> {
        const BRIEF: &str = "You revise a document exactly as instructed and change nothing else. Reply with the COMPLETE revised document from its first line to its last, including every part you did not change. Never reply with only the changed part. No preface, no commentary, no code fences.";
        let before = std::fs::read(self.root().join(path)).map_err(AdapterError::Io)?;
        let text = String::from_utf8_lossy(&before).into_owned();
        let Some(revised) = self.consult(gateway, Model::Glm53Flash, BRIEF, &format!("INSTRUCTION\n{instruction}\n\nDOCUMENT\n{text}"))? else {
            return Ok(None);
        };
        let mut content = revised.trim().to_owned();
        content.push('\n');
        Ok(Some(ChangeRequest {
            title: format!("Revise {path}"),
            reason: instruction.to_owned(),
            proposed_by: crate::RESEARCHER,
            files: Vec::new(),
            edits: vec![Edit { path: path.to_owned(), was: ContentDigest(rigor_waist::digest_bytes(&before)), content }],
        }))
    }

    /// Have the reviewing model read the change and acknowledge or object. `None` if the budget
    /// could not cover the consultation.
    ///
    /// # Errors
    /// On a gateway failure or a kernel fault.
    pub fn review_change(&mut self, gateway: &Gateway, id: &ChangeId) -> Result<Option<crate::research::Verdict>, StewardError> {
        const BRIEF: &str = "You are a strict independent reviewer of a proposed change to a project. You are shown its title, reason and the full content of every file it adds. Decide whether it should be accepted: it does what it says, is correct as far as you can tell, and would not mislead a reader. Your FIRST line must be exactly ACCEPT or REJECT, followed by at most five short bullet points.";
        if self.budget_left()? < 30_000 {
            return Ok(None);
        }
        let request = self.change(id)?;
        let mut shown = format!("TITLE\n{}\n\nREASON\n{}\n", request.title, request.reason);
        for f in &request.files {
            shown.push_str(&format!("\nNEW FILE {}\n{}\n", f.path, f.content.chars().take(12_000).collect::<String>()));
        }
        for e in &request.edits {
            let before = std::fs::read_to_string(self.root().join(&e.path)).unwrap_or_default();
            shown.push_str(&format!(
                "\nEDITED FILE {}\n--- BEFORE ---\n{}\n--- AFTER ---\n{}\n",
                e.path,
                before.chars().take(8_000).collect::<String>(),
                e.content.chars().take(8_000).collect::<String>()
            ));
        }
        let reply = gateway.ask(Model::Gpt6Luna, BRIEF, &shown, 3000)?;
        if !self.pay(reply.micro_usd.max(1))? {
            return Ok(None);
        }
        std::fs::write(self.changes_dir().join(format!("{id}.review.md")), &reply.text).map_err(AdapterError::Io)?;
        let prop = Prop::Sound(id.clone());
        // A new review supersedes the reviewer's earlier one: the earlier evidence is withdrawn.
        let on_it = self.world.epi()?.evidence_on(&prop);
        let earlier: Vec<_> = self.world.epi()?.evidence().filter(|e| on_it.contains(&e.id) && e.source == REVIEWER).map(|e| e.id).collect();
        for e in earlier {
            let _ = self.world.act(&REVIEWER, Act::Retract { evidence: e })?;
        }
        let said = crate::research::verdict(&reply.text);
        let accepted = match said {
            crate::research::Verdict::Accept => true,
            crate::research::Verdict::Reject => false,
            crate::research::Verdict::Unclear => {
                self.persist()?;
                return Ok(Some(said));
            }
        };
        let a = Attestation {
            about: if accepted { About::Supports(prop) } else { About::Rebuts(prop) },
            method: Method::Ack,
            derived_from: BTreeSet::new(),
            artifacts: BTreeMap::new(),
            content: Content::Document { title: "review of the change".to_owned(), digest: ContentDigest(rigor_waist::digest_bytes(reply.text.as_bytes())) },
        };
        let _ = self.world.act(&REVIEWER, Act::Attest(a))?;
        self.persist()?;
        Ok(Some(said))
    }

    /// The owner approves or rejects a change. Only ever called on the owner's instruction.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn owner_rules_on(&mut self, id: &ChangeId, approve: bool) -> Result<(), StewardError> {
        let prop = Prop::Sound(id.clone());
        let about = if approve { About::Supports(prop) } else { About::Rebuts(prop) };
        let _ = self.world.act(&OWNER, attest(about, Method::Audit, "the owner's ruling on this change"))?;
        self.persist()
    }
}
