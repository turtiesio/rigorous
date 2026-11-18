//! Where things are in the repository. The only place that names its directories.

/// Directories holding source code, outermost dependency first.
pub const CODE: [&str; 7] = ["waist", "logic", "concepts", "assembly", "claims", "edges", "operate"];

/// The directory holding the binaries and the tests.
pub const APP: &str = "app";

/// Top-level files that decide how the code is built.
pub const BUILD_FILES: [&str; 3] = ["Cargo.toml", "Cargo.lock", "rustfmt.toml"];

/// Directories of proofs and of documents.
pub const PROOFS: &str = "proofs";
pub const DOCS: &str = "docs";

/// Where a change the system proposes may add or edit a file. Code and proofs are outside:
/// nothing the system proposes can alter what it is trusted for.
pub const OPEN_TO_PROPOSALS: [&str; 2] = ["docs/research/", "app/tests/generated_"];
