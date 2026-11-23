//! A dotted package name resolves imports by its leading module segments.

use std::collections::HashMap;

use fensu_facts::extension::models::ProgramHandle;
use fensu_native::{
    evaluate_core_rules, plan_core_rule_queries, NativeProjectPlane, NativeRuleContext,
};
use ruff_python_ast::PythonVersion;

use crate::test_types::DottedPackageTestCase;

const CODE: &str = "FFL104";

#[test]
fn given_dotted_package_when_importing_private_main_then_domains_are_read_below_the_package() {
    let test_cases = [
        DottedPackageTestCase {
            description: "another domain's private main entry is rejected",
            relative_parts: &["inventory", "_helpers", "use.py"],
            source: "from app.nodes.orders.billing.main._calculate import calculate\n",
            expected_query_path: "src/app/nodes/orders/billing/main/_calculate.py",
            expected_messages: &[
                "import 'app.nodes.orders.billing.main._calculate' reaches a domain-private main \
                 entry",
            ],
        },
        DottedPackageTestCase {
            description: "the same domain's private main entry is allowed",
            relative_parts: &["orders", "shipping", "_helpers", "use.py"],
            source: "from app.nodes.orders.billing.main._calculate import calculate\n",
            expected_query_path: "src/app/nodes/orders/billing/main/_calculate.py",
            expected_messages: &[],
        },
    ];
    let version = PythonVersion {
        major: 3,
        minor: 12,
    };
    for test_case in &test_cases {
        let program = ProgramHandle::parse_many(vec![test_case.source.to_owned()], version)
            .pop()
            .flatten()
            .expect("fixture source parses");
        let codes = vec![CODE.to_owned()];
        let mut context = NativeRuleContext {
            scope: "root".to_owned(),
            package_name: "app.nodes".to_owned(),
            relative_parts: test_case
                .relative_parts
                .iter()
                .map(|part| (*part).to_owned())
                .collect(),
            repository_path: format!("src/app/nodes/{}", test_case.relative_parts.join("/")),
            scope_roots: vec![("root".to_owned(), "src/app/nodes".to_owned())],
            ownership_root: Some("/repo/src/app/nodes".to_owned()),
            ownership_roots: vec!["/repo/src/app/nodes".to_owned()],
            repo_root: "/repo".to_owned(),
            ..NativeRuleContext::default()
        };
        let queries = plan_core_rule_queries(&program, &codes, &context);
        let paths: Vec<&str> = queries.iter().map(|query| query.path.as_str()).collect();
        assert_eq!(
            paths,
            vec![test_case.expected_query_path],
            "{}",
            test_case.description
        );
        context.observations = queries
            .iter()
            .map(|query| (query.key(), vec!["true".to_owned()]))
            .collect::<HashMap<_, _>>();
        let project = NativeProjectPlane::new(Vec::new(), Vec::new());
        let messages: Vec<String> = evaluate_core_rules(&program, &codes, &context, &project)
            .expect("rules evaluate")
            .into_iter()
            .filter_map(|fault| fault.message)
            .collect();
        assert_eq!(
            messages, test_case.expected_messages,
            "{}",
            test_case.description
        );
    }
}
