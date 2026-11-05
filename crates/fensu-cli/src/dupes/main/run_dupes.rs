//! Run the advisory duplicated-code report for the current repository.

use std::env;
use std::io::{self, IsTerminal, Write};
use std::time::Instant;

use crate::dupes::_helpers::inputs::arguments::parse_arguments;
use crate::dupes::_helpers::inputs::sources::discover_workspace;
use crate::dupes::_helpers::report::build::build_report;
use crate::dupes::_helpers::report::rendering::{render_json, render_text};
use crate::dupes::constants::DUPES_HELP;
use crate::models::CliOutput;

/// Exit 0 when analysis succeeds, 1 when `--fail-on-new` finds a cluster, and 2 on errors.
pub(crate) fn run_dupes(arguments: &[String]) -> Result<CliOutput, String> {
    let Some(options) = parse_arguments(arguments)? else {
        return Ok(CliOutput::success(DUPES_HELP.to_owned()));
    };
    let started = Instant::now();
    if io::stderr().is_terminal() {
        let _ = writeln!(io::stderr(), "fensu dupes: detecting duplicated code ...");
    }
    let invocation = dunce::canonicalize(env::current_dir().map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())?;
    let report = discover_workspace(&invocation, &options)
        .and_then(|workspace| build_report(&workspace, &options))
        .map_err(|error| format!("fensu dupes: analysis failed: {error}"))?;
    let stdout = if options.json {
        render_json(&report, options.top)?
    } else {
        render_text(&report, options.top)
    };
    let units: usize = report.unit_counts.values().sum();
    let clusters = report.clusters.len();
    let plural = if clusters == 1 { "" } else { "s" };
    let mut stderr = format!(
        "fensu dupes: found {clusters} duplicated-code cluster{plural} across {units} units in {:.1}s",
        started.elapsed().as_secs_f64()
    );
    let failed = options.fail_on_new && clusters > 0;
    stderr.push_str(match (options.fail_on_new, failed) {
        (false, _) => " (advisory)\n",
        (true, false) => " (no new clusters)\n",
        (true, true) => "\n",
    });
    if failed {
        let verb = if clusters == 1 { "touches" } else { "touch" };
        stderr.push_str(&format!(
            "fensu dupes: failed: {clusters} new cluster{plural} {verb} changed lines; consolidate them or list them in [dupes] allowlist with a reason\n"
        ));
    }
    Ok(CliOutput {
        stdout,
        stderr,
        exit_code: i32::from(failed),
    })
}
