//! Analyze a Cargo workspace behind an owned diagnostic boundary.

use std::path::Path;

use crate::models::{RepositoryAnalysis, RepositoryAnalysisRequest};
use crate::types::ThresholdLookup;

/// Analyze a Cargo workspace using the normal Fensu rule options and path thresholds.
pub fn analyze_repository(
    repository_root: &Path,
    options: Option<&toml::map::Map<String, toml::Value>>,
    tooling: &[String],
    thresholds: &dyn ThresholdLookup,
) -> Result<RepositoryAnalysis, String> {
    crate::engine::_helpers::repository_analysis::analyze(RepositoryAnalysisRequest {
        repository_root,
        options,
        tooling,
        ownership_roots: &[],
        thresholds,
    })
}
