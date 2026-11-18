//! A real command as evidence: its exit status supports or rebuts a proposition.

use crate::error::AdapterError;
use rigor_institution::model::{About, Act, ActorId, ArtifactId, Attestation, Content, ContentDigest, Method, Prop};
use rigor_institution::world::World;
use rigor_waist::digest_bytes;
use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;

use crate::watch::ArtifactWatcher;

/// Run a real command and attest its outcome as evidence bound to the given artifacts.
/// Exit status 0 supports the proposition; anything else rebuts it.
///
/// # Errors
/// If the command cannot be run, an artifact is not watched, or on a kernel fault.
pub fn run_check(
    w: &mut World,
    watcher: &mut ArtifactWatcher,
    command: &mut Command,
    prop: &Prop,
    artifacts: &[ArtifactId],
    by: &ActorId,
) -> Result<bool, AdapterError> {
    let _ = watcher.scan(w)?;
    let output = command.output().map_err(AdapterError::Io)?;
    let passed = output.status.success();
    let mut bound = BTreeMap::new();
    for a in artifacts {
        let d = watcher.seen(a).ok_or_else(|| AdapterError::UnknownArtifact(a.clone()))?;
        let _ = bound.insert(a.clone(), d);
    }
    let mut bytes = output.stdout;
    bytes.extend(output.stderr);
    let attestation = Attestation {
        about: if passed { About::Supports(prop.clone()) } else { About::Rebuts(prop.clone()) },
        method: Method::Test,
        derived_from: BTreeSet::new(),
        artifacts: bound,
        content: Content::Command {
            program: command.get_program().to_string_lossy().into_owned(),
            exit: output.status.code().unwrap_or(-1),
            output: ContentDigest(digest_bytes(&bytes)),
        },
    };
    let _ = w.act(by, Act::Attest(attestation))?;
    Ok(passed)
}
