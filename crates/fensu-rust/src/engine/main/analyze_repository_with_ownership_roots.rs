//! Analyze a Cargo workspace with explicit ownership roots.

use crate::models::{RepositoryAnalysis, RepositoryAnalysisRequest};

/// Analyze a Cargo workspace using concrete repository-relative ownership roots.
pub fn analyze_repository_with_ownership_roots(
    request: RepositoryAnalysisRequest<'_>,
) -> Result<RepositoryAnalysis, String> {
    crate::engine::_helpers::repository_analysis::analyze(request)
}
