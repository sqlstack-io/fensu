//! Code-line counting excludes blank and comment-only lines.

use fensu_facts::ProgramHandle;

use crate::test_types;

#[test]
fn given_source_shapes_when_counting_code_lines_then_blanks_and_comments_are_excluded() {
    let test_cases = [
        test_types::CodeLineCountTestCase {
            description: "blank and comment-only lines do not count",
            source: "# heading\n\nvalue = 1\n\n    # indented comment\nother = 2\n",
            expected_code_lines: 2,
        },
        test_types::CodeLineCountTestCase {
            description: "code with a trailing comment counts",
            source: "value = 1  # explained\n",
            expected_code_lines: 1,
        },
        test_types::CodeLineCountTestCase {
            description: "docstrings count as code",
            source: "def load() -> int:\n    \"\"\"Load the order count.\"\"\"\n    return 1\n",
            expected_code_lines: 3,
        },
        test_types::CodeLineCountTestCase {
            description: "every line of a multi-line string counts, including blank ones",
            source: "text = \"\"\"first\n\nthird\"\"\"\n",
            expected_code_lines: 3,
        },
        test_types::CodeLineCountTestCase {
            description: "backslash continuation lines count",
            source: "total = 1 + \\\n    2\n",
            expected_code_lines: 2,
        },
        test_types::CodeLineCountTestCase {
            description: "form-feed separator lines do not count",
            source: "first = 1\n\x0c\nsecond = 2\n",
            expected_code_lines: 2,
        },
        test_types::CodeLineCountTestCase {
            description: "empty module holds no code",
            source: "",
            expected_code_lines: 0,
        },
    ];
    for test_case in &test_cases {
        let program =
            ProgramHandle::parse_source(test_case.source, 3, 12).expect("code-line fixture parses");
        assert_eq!(
            program.code_line_count(),
            test_case.expected_code_lines,
            "{}",
            test_case.description
        );
    }
}
