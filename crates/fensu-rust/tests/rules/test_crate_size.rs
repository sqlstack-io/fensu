//! Crate code-line budget behavior over fixture workspaces.

use crate::helpers;
use crate::test_types;

#[test]
fn given_crate_code_line_budget_when_checking_then_counts_only_non_test_code() {
    let test_cases = [
        test_types::CrateSizeTestCase {
            description: "a crate at its code-line limit is accepted",
            repo_files: helpers::counted_crate_files(),
            max_crate_code_lines: 6,
            expected_messages: vec![],
        },
        test_types::CrateSizeTestCase {
            description: "a crate over its code-line limit is reported at its manifest",
            repo_files: helpers::counted_crate_files(),
            max_crate_code_lines: 5,
            expected_messages: vec![
                "crate holds 6 code lines (limit 5); split it into crates by area.",
            ],
        },
        test_types::CrateSizeTestCase {
            description: "large counts and limits are grouped by thousands",
            repo_files: vec![test_types::RepoFile {
                path: "crates/example/src/lib.rs".to_owned(),
                contents: "pub const VALUE: usize = 1;\n".repeat(1001),
            }],
            max_crate_code_lines: 1000,
            expected_messages: vec![
                "crate holds 1,001 code lines (limit 1,000); split it into crates by area.",
            ],
        },
    ];
    for test_case in &test_cases {
        let repo_root = helpers::write_temp_repo_verbatim(&test_types::CheckRepoTestCase {
            description: test_case.description,
            repo_files: test_case.repo_files.clone(),
            expected_violation_codes: Vec::new(),
        });
        let mut policy = fensu_rust::models::RustPolicy::default();
        policy.repository.thresholds.max_crate_code_lines = test_case.max_crate_code_lines;

        let violations =
            fensu_rust::rules::main::check_repository_with_config::check_repository_with_config(
                &repo_root, &policy,
            )
            .expect("fixture rule policy is valid");

        helpers::remove_temp_repo(&repo_root);
        let reported = violations
            .iter()
            .filter(|violation| violation.code == "RSR602")
            .map(|violation| {
                assert_eq!(
                    violation.path.to_string_lossy(),
                    "crates/example/Cargo.toml",
                    "case failed: {}",
                    test_case.description
                );
                violation.message.as_str()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            reported, test_case.expected_messages,
            "case failed: {}",
            test_case.description
        );
    }
}
