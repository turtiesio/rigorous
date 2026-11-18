//! Research under the institution's own rules.
//!
//! A question is put to one model (the researcher). Its answer is a CLAIM about a document:
//! "this document adequately answers the question". A second, different model (the reviewer)
//! reads the answer and either acknowledges it or objects. Only an acknowledged answer is
//! accepted, and "the question is answered" is then derived from that acknowledgement, so it
//! falls if the acknowledgement is ever withdrawn. Every consultation is paid for out of the
//! owner's budget, through the ledger.
//!
//! The agenda is about managing work in general. Software is the first case, not the boundary.

use crate::selfmodel::SelfReport;
use crate::{Steward, StewardError, CI, RESEARCHER, REVIEWER};
use rigor_institution::model::{About, Act, Assessment, Attestation, Content, ContentDigest, Event, Method, Prop};
use rigor_kernel::{digest_bytes, Name};
use rigor_mind::{Gateway, Model};
use std::collections::{BTreeMap, BTreeSet};

/// A question the system has set itself.
#[derive(Clone, Debug)]
pub struct Question {
    pub id: Name,
    pub text: &'static str,
}

/// The standing agenda.
#[must_use]
pub fn agenda() -> Vec<Question> {
    let q = |id: &'static str, text| Question { id: Name::lit(id), text };
    vec![
        q("what-to-track-to-manage-change", "What is the smallest set of things that must be recorded, and kept connected, for changes to a body of work to be managed well: so that at any moment one can say what was asked for, what was done, on what evidence it is believed done, who answers for it, and what else is affected? Give the set, and for each item what goes wrong without it."),
        q("how-to-decide-what-to-defer", "By what explicit criteria should work be sorted into: do now, defer with a stated trigger, and deliberately not do? What makes a deferral honest instead of a way of losing things?"),
        q("what-an-operator-must-see", "A person is responsible for a system that largely runs itself and records its own obligations, evidence, abilities and limits. What must one screen show that person so they can tell in a minute whether anything needs them, and what must it never hide?"),
        q("managing-work-beyond-software", "Which practices for managing software work (review, tests as evidence, staged rollout, incident review, change approval) carry over to work that is not software, such as procurement, contracts or operations, and which do not? Be specific about what replaces a test."),
        q("how-a-system-should-know-its-limits", "How should a system that governs work represent what it cannot do, what it would need in order to do it, and what it must never do, so that the representation stays true as the system changes?"),
        q("what-should-change-next", "Given this system's own account of what it can and cannot do, and its open obligations, what are the three most valuable changes to make next? For each give a one-line reason and what evidence would show it is done."),
    ]
}

/// What a reviewer's reply says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Accept,
    Reject,
    /// The reply was empty or did not begin with a verdict. That is not an objection: it is
    /// nothing, and nothing is recorded for it.
    Unclear,
}

/// Read a reviewer's reply: its first non-empty line must begin with ACCEPT or REJECT.
#[must_use]
pub fn verdict(reply: &str) -> Verdict {
    match reply.lines().map(str::trim).find(|l| !l.is_empty()) {
        Some(l) if l.starts_with("ACCEPT") => Verdict::Accept,
        Some(l) if l.starts_with("REJECT") => Verdict::Reject,
        Some(_) | None => Verdict::Unclear,
    }
}

/// What became of one round of research on a question.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Already answered (an accepted answer stands): nothing was asked.
    AlreadyAnswered,
    Accepted {
        rounds: u8,
        answer: ContentDigest,
    },
    /// The reviewer still objected after the allowed rounds.
    NotAccepted {
        rounds: u8,
    },
    /// The reviewer gave no usable verdict; nothing was recorded for or against.
    NoVerdict,
    /// The budget ran out before the work was done.
    OutOfBudget,
}

const RESEARCHER_BRIEF: &str = "You are a careful researcher answering one question for the owner of a system that governs work. Answer in plain, direct prose with short headings and lists. Be concrete and specific; prefer established practice to novelty; say what you are unsure of. At most 450 words. Do not flatter and do not restate the question.";
const REVIEWER_BRIEF: &str = "You are a strict independent reviewer. You are given a question and an answer written by someone else. Decide whether the answer is adequate: correct as far as you can tell, specific, and actually answering the question. Your FIRST line must be exactly ACCEPT or REJECT. Then give at most five short bullet points: the reasons, or the specific objections that would have to be met.";

/// The proposition "this question has an accepted answer".
#[must_use]
pub fn answered(question: &Name) -> Prop {
    Prop::Answered(question.clone())
}

impl Steward {
    /// Ask a model, and pay for it out of the budget. `None` if the budget cannot cover it.
    pub(crate) fn consult(&mut self, gateway: &Gateway, model: Model, brief: &str, prompt: &str) -> Result<Option<String>, StewardError> {
        // Never start a consultation that the remaining budget could not pay for.
        const WORST_CASE: u64 = 30_000;
        if self.budget_left()? < WORST_CASE {
            return Ok(None);
        }
        let reply = gateway.ask(model, brief, prompt, 3000)?;
        // A reply that cannot be paid for is not used.
        Ok(self.pay(reply.micro_usd.max(1))?.then_some(reply.text))
    }

    /// Research one question: draft, review, and at most two revisions.
    ///
    /// # Errors
    /// On a gateway failure, I/O failure or a kernel fault.
    pub fn research(&mut self, gateway: &Gateway, q: &Question, about_itself: &SelfReport) -> Result<Outcome, StewardError> {
        let done = answered(&q.id);
        if self.world.epi()?.assessment(&done) == Assessment::Accepted {
            return Ok(Outcome::AlreadyAnswered);
        }
        // An answer already accepted on review, from which "answered" has not yet been drawn
        // (for example in a record made before that inference had its present form): draw it,
        // without consulting anyone again.
        let standing: Vec<_> = self
            .world
            .epi()?
            .standings()
            .filter(|(p, s)| matches!(p, Prop::Answers { question, .. } if *question == q.id) && s.assessment == Assessment::Accepted)
            .flat_map(|(_, s)| s.basis.clone())
            .collect();
        if !standing.is_empty() {
            let derived = Attestation {
                about: About::Supports(done),
                method: Method::Inference,
                derived_from: standing.into_iter().collect(),
                artifacts: BTreeMap::new(),
                content: Content::Note(format!("answer to {} already accepted on review", q.id)),
            };
            let _ = self.world.act(&CI, Act::Attest(derived))?;
            self.persist()?;
            return Ok(Outcome::AlreadyAnswered);
        }
        let dir = self.home().join("research");
        std::fs::create_dir_all(&dir).map_err(rigor_adapter_host::AdapterError::Io)?;
        let context = format!("QUESTION\n{}\n\nTHE SYSTEM'S OWN ACCOUNT OF ITSELF (for context)\n{}", q.text, about_itself.describe());
        let mut objections = String::new();
        for round in 1..=3u8 {
            let prompt = if objections.is_empty() {
                context.clone()
            } else {
                format!("{context}\n\nA REVIEWER REJECTED YOUR PREVIOUS ANSWER FOR THESE REASONS. Write a new answer that meets them.\n{objections}")
            };
            let Some(answer) = self.consult(gateway, Model::Glm53Flash, RESEARCHER_BRIEF, &prompt)? else { return Ok(Outcome::OutOfBudget) };
            let digest = ContentDigest(digest_bytes(answer.as_bytes()));
            std::fs::write(dir.join(format!("{}.md", q.id)), &answer).map_err(rigor_adapter_host::AdapterError::Io)?;
            let claim = Prop::Answers { question: q.id.clone(), answer: digest };
            let _ = self.world.act(&RESEARCHER, Act::Claim { prop: claim.clone() })?;

            let Some(review) = self.consult(gateway, Model::Gpt6Luna, REVIEWER_BRIEF, &format!("QUESTION\n{}\n\nANSWER\n{answer}", q.text))? else {
                return Ok(Outcome::OutOfBudget);
            };
            std::fs::write(dir.join(format!("{}.review-{round}.md", q.id)), &review).map_err(rigor_adapter_host::AdapterError::Io)?;
            let accepted = match verdict(&review) {
                Verdict::Accept => true,
                Verdict::Reject => false,
                Verdict::Unclear => {
                    self.persist()?;
                    return Ok(Outcome::NoVerdict);
                }
            };
            let about = if accepted { About::Supports(claim) } else { About::Rebuts(claim) };
            let a = Attestation {
                about,
                method: Method::Ack,
                derived_from: BTreeSet::new(),
                artifacts: BTreeMap::new(),
                content: Content::Document { title: format!("review of the answer to {}", q.id), digest: ContentDigest(digest_bytes(review.as_bytes())) },
            };
            let entries = self.world.act(&REVIEWER, Act::Attest(a))?;
            if accepted {
                // "The question is answered" rests on the reviewer's acknowledgement and falls with it.
                let basis: BTreeSet<_> = entries
                    .iter()
                    .filter_map(|e| match &e.event {
                        Event::EvidenceRecorded(ev) => Some(ev.id),
                        _ => None,
                    })
                    .collect();
                let derived = Attestation {
                    about: About::Supports(done),
                    method: Method::Inference,
                    derived_from: basis,
                    artifacts: BTreeMap::new(),
                    content: Content::Document { title: format!("answer to {}", q.id), digest },
                };
                let _ = self.world.act(&CI, Act::Attest(derived))?;
                self.persist()?;
                return Ok(Outcome::Accepted { rounds: round, answer: digest });
            }
            objections = review;
        }
        self.persist()?;
        Ok(Outcome::NotAccepted { rounds: 3 })
    }
}
