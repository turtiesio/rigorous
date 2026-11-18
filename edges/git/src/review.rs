//! Reviews from a forge, attached to the commits they concern.

use crate::commit::*;
use crate::error::*;
use rigor_institution::model::{ActorId, ChangeId};
use rigor_waist::{Name, NameError};

/// What a forge recorded about the review of one merged change (pseudonymised).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Review {
    pub change: ChangeId,
    pub approvers: Vec<ActorId>,
    pub requested_changes: Vec<ActorId>,
}

/// Parse forge review records: one line per merged pull request,
/// `<merge commit> <author id> <approver ids or -> <ids that requested changes or ->`.
///
/// # Errors
/// If a line is malformed.
pub fn parse_reviews(text: &str) -> Result<Vec<Review>, GitError> {
    let ids = |s: &str| -> Result<Vec<ActorId>, NameError> {
        if s == "-" {
            Ok(Vec::new())
        } else {
            s.split(',').map(|x| Name::new(x).map(ActorId)).collect()
        }
    };
    let mut out = Vec::new();
    for (record, line) in text.lines().enumerate() {
        let fields: Vec<&str> = line.split(' ').collect();
        let [change, _author, approvers, objectors] = fields.as_slice() else {
            return Err(GitError::Malformed { record });
        };
        out.push(Review { change: ChangeId(Name::new(change)?), approvers: ids(approvers)?, requested_changes: ids(objectors)? });
    }
    Ok(out)
}

/// Attach forge reviews to the commits they are about: approvers become reviewers; those who
/// requested changes and never approved become objectors. Returns how many reviews found
/// their commit.
pub fn apply_reviews(commits: &mut [Commit], reviews: &[Review]) -> usize {
    let mut matched = 0;
    for r in reviews {
        if let Some(c) = commits.iter_mut().find(|c| c.id == r.change) {
            matched += 1;
            for a in r.approvers.iter().filter(|a| **a != c.author.id) {
                if !c.reviewers.contains(a) {
                    c.reviewers.push(a.clone());
                }
            }
            for o in r.requested_changes.iter().filter(|o| **o != c.author.id && !r.approvers.contains(o)) {
                if !c.objectors.contains(o) {
                    c.objectors.push(o.clone());
                }
            }
        }
    }
    matched
}
