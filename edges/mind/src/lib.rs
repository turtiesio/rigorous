//! Adapter: language models, reached through an AI gateway (OpenAI-compatible chat API).
//!
//! A model is an outside party like any other. What it returns is text that somebody said:
//! in the institution it can only ever become a CLAIM or a PROPOSAL by the member that asked,
//! never evidence by itself. This crate knows nothing of institutions; it sends a prompt and
//! returns the reply together with what it cost.
//!
//! The credential is read at the moment of use, from the environment or from a key file, and is
//! never stored, logged or returned.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// The models this system is configured to consult.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Model {
    /// Fast and cheap: first drafts.
    Glm53Flash,
    /// Stronger: review and second opinions.
    Gpt6Luna,
}

impl Model {
    /// The identifier the gateway knows it by.
    #[must_use]
    pub const fn gateway_id(self) -> &'static str {
        match self {
            Self::Glm53Flash => "zai/glm-5.3-flash",
            Self::Gpt6Luna => "openai/gpt-6-luna",
        }
    }
}

/// Where the gateway credential may be found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeySource {
    /// The environment variable `AI_GATEWAY_API_KEY`.
    Environment,
    /// A JSON file of the form `{"vercel-ai-gateway": {"key": "..."}}`.
    File(PathBuf),
}

/// Why a consultation did not produce a reply.
#[derive(Debug)]
pub enum MindError {
    /// No credential in any configured source.
    NoCredential,
    /// The gateway could not be reached.
    Unreachable(String),
    /// The gateway answered with an error status.
    Refused { status: u16, body: String },
    /// The reply did not have the expected shape.
    Malformed(String),
}

/// A reply and what it cost.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reply {
    pub text: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    /// Cost as reported by the gateway, in millionths of a US dollar, rounded up.
    pub micro_usd: u64,
}

/// A connection to the gateway.
#[derive(Clone, Debug)]
pub struct Gateway {
    base: String,
    sources: Vec<KeySource>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'static str,
    content: &'a str,
}

#[derive(Serialize)]
struct Request<'a> {
    model: &'static str,
    messages: Vec<Message<'a>>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}
#[derive(Deserialize)]
struct Choice {
    message: ReplyMessage,
}
#[derive(Deserialize)]
struct ReplyMessage {
    content: Option<String>,
}
#[derive(Deserialize)]
struct Usage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    cost: Option<f64>,
}

#[derive(Deserialize)]
struct KeyFile {
    #[serde(rename = "vercel-ai-gateway")]
    gateway: KeyEntry,
}
#[derive(Deserialize)]
struct KeyEntry {
    key: String,
}

impl Gateway {
    /// The Vercel AI Gateway, with the credential looked for in the environment and then in
    /// the given key files.
    #[must_use]
    pub fn vercel(key_files: Vec<PathBuf>) -> Self {
        let mut sources = vec![KeySource::Environment];
        sources.extend(key_files.into_iter().map(KeySource::File));
        Self { base: "https://ai-gateway.vercel.sh/v1".to_owned(), sources }
    }

    fn key(&self) -> Result<String, MindError> {
        for s in &self.sources {
            let found = match s {
                KeySource::Environment => std::env::var("AI_GATEWAY_API_KEY").ok(),
                KeySource::File(p) => std::fs::read_to_string(p).ok().and_then(|t| serde_json::from_str::<KeyFile>(&t).ok()).map(|k| k.gateway.key),
            };
            if let Some(k) = found.filter(|k| !k.trim().is_empty()) {
                return Ok(k);
            }
        }
        Err(MindError::NoCredential)
    }

    /// Whether a credential can be found (without revealing or using it).
    #[must_use]
    pub fn has_credential(&self) -> bool {
        self.key().is_ok()
    }

    /// Whether the gateway answers at all with this credential.
    ///
    /// # Errors
    /// See [`MindError`].
    pub fn reachable(&self) -> Result<(), MindError> {
        let key = self.key()?;
        match ureq::get(&format!("{}/models", self.base)).set("Authorization", &format!("Bearer {key}")).timeout(Duration::from_secs(20)).call() {
            Ok(_) => Ok(()),
            Err(ureq::Error::Status(status, r)) => Err(MindError::Refused { status, body: r.into_string().unwrap_or_default().chars().take(300).collect() }),
            Err(e) => Err(MindError::Unreachable(e.to_string())),
        }
    }

    /// Ask a model one question under a system instruction.
    ///
    /// # Errors
    /// See [`MindError`].
    pub fn ask(&self, model: Model, system: &str, user: &str, max_tokens: u32) -> Result<Reply, MindError> {
        let key = self.key()?;
        let body = Request {
            model: model.gateway_id(),
            messages: vec![Message { role: "system", content: system }, Message { role: "user", content: user }],
            max_tokens,
            temperature: 0.2,
        };
        let sent = ureq::post(&format!("{}/chat/completions", self.base))
            .set("Authorization", &format!("Bearer {key}"))
            .timeout(Duration::from_secs(180))
            .send_json(&body);
        let response: Response = match sent {
            Ok(r) => r.into_json().map_err(|e| MindError::Malformed(e.to_string()))?,
            Err(ureq::Error::Status(status, r)) => {
                return Err(MindError::Refused { status, body: r.into_string().unwrap_or_default().chars().take(300).collect() })
            }
            Err(e) => return Err(MindError::Unreachable(e.to_string())),
        };
        let text = response.choices.into_iter().next().and_then(|c| c.message.content).ok_or_else(|| MindError::Malformed("no reply text".to_owned()))?;
        let usage = response.usage;
        let cost = usage.as_ref().and_then(|u| u.cost).unwrap_or(0.0);
        // Round up to whole millionths: never under-record spending.
        let micro = format!("{:.0}", (cost * 1_000_000.0).ceil().max(0.0)).parse::<u64>().unwrap_or(u64::MAX);
        Ok(Reply {
            text,
            prompt_tokens: usage.as_ref().and_then(|u| u.prompt_tokens).unwrap_or(0),
            completion_tokens: usage.as_ref().and_then(|u| u.completion_tokens).unwrap_or(0),
            micro_usd: micro,
        })
    }
}
