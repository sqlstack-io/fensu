use crate::helpers::{run_check, run_check_with, write};
use crate::test_types::SizeLimitCheckTestCase;

const PYTHON_BUCKETS: &[(&str, &str)] = &[
    ("src/pkg/orders/_helpers/__init__.py", ""),
    ("src/pkg/orders/_helpers/pricing/rates.py", ""),
    ("src/pkg/orders/_helpers/shipping/zones.py", ""),
    ("src/pkg/orders/_helpers/tax/regions.py", ""),
    ("src/pkg/orders/_helpers/assets/rates.json", "{}"),
];

const PYTHON_PACKAGE: &[(&str, &str)] = &[
    ("src/pkg/__init__.py", ""),
    ("src/pkg/orders/__init__.py", "\"\"\"Orders.\"\"\"\n"),
    (
        "src/pkg/orders/models.py",
        "# Order totals.\n\ncount: int = 1\nlimit: int = 2\n\nlabel: str = 'orders'  # shown\n",
    ),
];

#[test]
fn given_python_size_limits_when_checking_natively_then_effective_limit_is_reported() {
    let test_cases = [
        SizeLimitCheckTestCase {
            description: "helpers container over its bucket limit is reported once",
            config: "roots = [\"src/pkg\"]\nselect = [\"FFR312\"]\n[thresholds]\nmax_helpers_container_buckets = 2\n",
            files: PYTHON_BUCKETS,
            expected_exit_code: 1,
            expected_present: &["--> src/pkg/orders/_helpers/__init__.py", "FFR312  _helpers/ holds 3 buckets (limit 2); split the owning module by area or move cohesive groups into their own module or crate."],
            expected_absent: "Threshold override:",
        },
        SizeLimitCheckTestCase {
            description: "helpers container at an overridden bucket limit passes",
            config: "roots = [\"src/pkg\"]\nselect = [\"FFR312\"]\n[thresholds]\nmax_helpers_container_buckets = 2\n[[threshold_overrides]]\npaths = [\"src/pkg/orders/_helpers/__init__.py\"]\nreason = \"Order helpers are being split.\"\nthresholds = { max_helpers_container_buckets = 3 }\n",
            files: PYTHON_BUCKETS,
            expected_exit_code: 0,
            expected_present: &["Threshold override: max_helpers_container_buckets=3", "Found 0 faults"],
            expected_absent: "FFR312  ",
        },
        SizeLimitCheckTestCase {
            description: "package over its code-line limit is reported at the root anchor",
            config: "roots = [\"src/pkg\"]\nselect = [\"FFR602\"]\n[thresholds]\nmax_crate_code_lines = 3\n",
            files: PYTHON_PACKAGE,
            expected_exit_code: 1,
            expected_present: &["--> src/pkg/__init__.py", "FFR602  package holds 4 code lines (limit 3); split it into packages by area."],
            expected_absent: "Threshold override:",
        },
        SizeLimitCheckTestCase {
            description: "package at an overridden code-line limit passes",
            config: "roots = [\"src/pkg\"]\nselect = [\"FFR602\"]\n[thresholds]\nmax_crate_code_lines = 3\n[[threshold_overrides]]\npaths = [\"src/pkg/**\"]\nreason = \"The package is being split.\"\nthresholds = { max_crate_code_lines = 4 }\n",
            files: PYTHON_PACKAGE,
            expected_exit_code: 0,
            expected_present: &["Threshold override: max_crate_code_lines=4", "Found 0 faults"],
            expected_absent: "FFR602  ",
        },
    ];
    for test_case in &test_cases {
        let repository = tempfile::tempdir().expect("temporary repository");
        write(repository.path().join("fensu.toml"), test_case.config);
        for (path, source) in test_case.files {
            write(repository.path().join(path), source);
        }

        let output = run_check(repository.path());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let cold = run_check_with(repository.path(), &["--cache"]);
        let warm = run_check_with(repository.path(), &["--cache"]);

        assert_eq!(
            output.status.code(),
            Some(test_case.expected_exit_code),
            "{}: {stdout} {}",
            test_case.description,
            String::from_utf8_lossy(&output.stderr)
        );
        for expected in test_case.expected_present {
            assert!(
                stdout.contains(expected),
                "{}: {stdout}",
                test_case.description
            );
        }
        assert!(
            !stdout.contains(test_case.expected_absent),
            "{}: {stdout}",
            test_case.description
        );
        assert_eq!(cold.stdout, warm.stdout, "{}", test_case.description);
        assert_eq!(cold.stdout, output.stdout, "{}", test_case.description);
    }
}
