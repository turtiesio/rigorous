//! The roadmap: what is owed, and by when it is to be reviewed.

use rigor_institution::model::Requirement;

/// An open item of the roadmap: something the project has undertaken to resolve or re-decide.
#[derive(Clone, Debug)]
pub struct BacklogItem {
    pub requirement: Requirement,
    /// Days after founding by which it must be done or consciously re-planned.
    pub review_within: u64,
}

/// The roadmap's "address next" list (docs/05-roadmap.md), as obligations. An item is
/// undertaken the first time the steward runs with it in this list; items that were closed
/// earlier stay in the log as satisfied commitments.
pub const BACKLOG: [BacklogItem; 14] = [
    BacklogItem { requirement: Requirement::lit("snapshots-are-attested-not-trusted"), review_within: 45 },
    BacklogItem { requirement: Requirement::lit("forge-review-evidence-adapter"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("real-data-for-transfers-and-budgets"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("expulsion-and-role-change"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("organisations-interacting-across-boundaries"), review_within: 75 },
    BacklogItem { requirement: Requirement::lit("a-second-kind-of-real-record"), review_within: 75 },
    BacklogItem { requirement: Requirement::lit("open-vocabulary-by-type-parameters"), review_within: 90 },
    BacklogItem { requirement: Requirement::lit("forge-reviews-at-scale-not-a-sample"), review_within: 90 },
    BacklogItem { requirement: Requirement::lit("a-human-in-the-loop-for-reflector-proposals"), review_within: 90 },
    BacklogItem { requirement: Requirement::lit("kernel-verified-against-lean-model"), review_within: 180 },
    // Recommended by the system's own research ("what-should-change-next") and adopted by the owner.
    BacklogItem { requirement: Requirement::lit("controlled-editing-of-existing-files"), review_within: 30 },
    BacklogItem { requirement: Requirement::lit("controlled-changes-to-code-not-only-notes"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("capabilities-prohibitions-and-lacks-as-distinct-records"), review_within: 60 },
    BacklogItem { requirement: Requirement::lit("checks-that-cover-documents-not-only-code"), review_within: 45 },
];
