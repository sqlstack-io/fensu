//! Render one aggregate set of analyzer check results, marked incomplete when a target could not run.

use std::path::Path;

use crate::check::models::CheckResult;

pub(crate) fn render_check_results(
    results: Vec<CheckResult>,
    root: &Path,
    color: bool,
    (show_warnings, incomplete): (bool, bool),
) -> (String, i32) {
    crate::check::_helpers::evaluation::render_results(
        results,
        root,
        color,
        (show_warnings, incomplete),
    )
}
