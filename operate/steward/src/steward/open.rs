//! Opening the record: restore it, migrate it, re-found it, or begin; and check it against the anchor.

use super::*;

impl Steward {
    /// Open the steward's world: replay the stored log if there is a usable one, else found anew.
    ///
    /// # Errors
    /// On I/O failure or a kernel fault.
    pub fn open(root: &Path, log: &Path, day: u64) -> Result<Self, StewardError> {
        let mut rejected = None;
        let mut migrated = None;
        let world = if log.exists() {
            match restore(root_unit()?, log) {
                Ok(w) => w,
                Err(AdapterError::Io(e)) => return Err(StewardError::Adapter(AdapterError::Io(e))),
                // Written under an older format: its chain is verified and its history carried over.
                Err(AdapterError::Log(LogError::Schema { found })) => match migrate(root_unit()?, log) {
                    Ok(w) => {
                        let kept = log.with_extension(format!("schema-{found}.jsonl"));
                        let _ = std::fs::copy(log, &kept).map_err(AdapterError::Io)?;
                        migrated = Some(found);
                        w
                    }
                    Err(AdapterError::Io(e)) => return Err(StewardError::Adapter(AdapterError::Io(e))),
                    Err(_) => {
                        let aside = log.with_extension(format!("rejected-day-{day}.jsonl"));
                        std::fs::rename(log, &aside).map_err(AdapterError::Io)?;
                        rejected = Some(aside);
                        found_world(day)?
                    }
                },
                Err(_) => {
                    // Tampered with, or written by an incompatible version of the vocabulary.
                    // History is never silently discarded: it is set aside, and a new one begins.
                    let aside = log.with_extension(format!("rejected-day-{day}.jsonl"));
                    std::fs::rename(log, &aside).map_err(AdapterError::Io)?;
                    rejected = Some(aside);
                    found_world(day)?
                }
            }
        } else {
            found_world(day)?
        };
        // A log founded before the project had an owner, models and a budget is carried over
        // under the current constitution, in the open (the old file is kept).
        let mut refounded = false;
        let world = if log.exists() && !world.inst()?.valid(&rigor_institution::model::CapId::Base(OWNER)) {
            let kept = log.with_extension(format!("before-refounding-day-{day}.jsonl"));
            let _ = std::fs::copy(log, &kept).map_err(AdapterError::Io)?;
            if migrated.is_some() {
                // The migrated world is only in memory: write it so that re-founding reads current-schema bytes.
                save(&world, log)?;
            }
            refounded = true;
            refound(root_unit()?, log, &constitution())?
        } else {
            world
        };
        // Measure the record against its anchor, unless it was just rebuilt in the open.
        let anchor: Option<Anchor> = std::fs::read_to_string(root.join(".rigor-anchor")).ok().and_then(|t| serde_json::from_str(&t).ok());
        let anchor_problem = match anchor {
            Some(a) if migrated.is_none() && !refounded && rejected.is_none() => {
                let found = world.rt.log().next_seq();
                let at_anchor = a.events.checked_sub(1).and_then(|i| usize::try_from(i).ok()).and_then(|i| world.rt.log().entries().get(i));
                if found < a.events {
                    Some(AnchorProblem::Shorter { anchored: a.events, found })
                } else if at_anchor.is_some_and(|e| e.id != a.head) {
                    Some(AnchorProblem::Different)
                } else {
                    None
                }
            }
            Some(_) | None => None,
        };
        let mut rust: Vec<PathBuf> = layout::BUILD_FILES.iter().map(|f| root.join(f)).collect();
        // A layer that holds nothing yet is simply absent.
        for dir in layout::CODE.iter().chain([&layout::APP]).map(|d| root.join(d)).filter(|d| d.is_dir()) {
            rust.extend(files_under(&dir, &[])?);
        }
        let watcher = ArtifactWatcher::of_sets([(SOURCES, rust), (PROOFS, files_under(&root.join(layout::PROOFS), &["lean"])?)]);
        Ok(Self {
            world,
            root: root.to_path_buf(),
            log: log.to_path_buf(),
            watcher,
            previous_log_rejected: rejected,
            migrated_from: migrated,
            refounded,
            anchor_problem,
        })
    }
}
