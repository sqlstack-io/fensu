# fensu-native

[Fensu](https://github.com/sqlstack-io/fensu)'s Python rule engine: core rules and dead-code analysis over programs parsed by `fensu-facts`.

```toml
fensu-facts = { version = "0.30", default-features = false }
fensu-native = { version = "0.30", default-features = false }
```

`default-features = false` leaves out the Python bindings and the cache, and their SQLite and compression dependencies.

The public API is at the crate root: `evaluate_core_rules`, `evaluate_dead_code`, `parse_dead_code`, `plan_core_rule_queries`, `plan_execution_owners`, `generated_policy_owners`, and their models such as `NativeRuleContext`, `NativeProjectPlane` and `NativeFaultRow`. Rule codes are in `rules::constants`.

```rust
use fensu_facts::ProgramHandle;
use fensu_native::{evaluate_core_rules, NativeProjectPlane, NativeRuleContext};
use fensu_native::rules::constants::PARAMETER_ANNOTATION_CODE;

let program = ProgramHandle::parse_source("def total(items):\n    return items\n", 3, 12)?;
let faults = evaluate_core_rules(
    &program,
    &[PARAMETER_ANNOTATION_CODE.to_owned()],
    &NativeRuleContext::default(),
    &NativeProjectPlane::new(Vec::new(), Vec::new()),
)?;
```
