//! Rust source roles and caller-supplied threshold resolution.

/// Resolve one threshold by name for one repository-relative path; `None` keeps the policy value.
pub trait ThresholdLookup: std::fmt::Debug {
    fn threshold(&self, name: &str, path: &str) -> Option<usize>;
}

/// The structural role a checked file plays inside its crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    LibraryRoot,
    BinAdapter,
    ModRoot,
    ModuleFile,
    TestHarness,
    TestTypes,
    TestHelpers,
    TestTopic,
}
