//! Internal entry point for checking one already-discovered Cargo workspace.

use std::path::Path;

use crate::models::{RustPolicy, Violation, WorkspaceScan};
use crate::types::ThresholdLookup;

pub(crate) fn check_scanned_workspace(
    repository_root: &Path,
    workspace: WorkspaceScan,
    config: &RustPolicy,
    lookup: &dyn ThresholdLookup,
) -> Vec<Violation> {
    crate::rules::_helpers::repository::project_checks::check_scan(
        repository_root,
        workspace,
        config,
        lookup,
    )
}
