#!/usr/bin/env bash
set -euo pipefail

package_root="$(mktemp -d)"
consumer_root="$(mktemp -d)"
trap 'rm -rf "${package_root}" "${consumer_root}"' EXIT

version="$(cargo +1.85.0 metadata --locked --no-deps --format-version 1 \
  | jq -r '.packages[] | select(.name == "fensu-policy") | .version')"

CARGO_TARGET_DIR="${package_root}" cargo +1.85.0 package --locked -p fensu-policy

cargo +stable new --quiet --lib "${consumer_root}/consumer"
cargo +stable add --quiet --manifest-path "${consumer_root}/consumer/Cargo.toml" \
  --path "${package_root}/package/fensu-policy-${version}"
cargo +1.85.0 check --locked --manifest-path "${consumer_root}/consumer/Cargo.toml"

# The Python analysis crates are packaged together, as fensu-native depends on fensu-facts. A fresh
# project then runs the core rules without default features, and must not pull in the Python
# bindings or the cache's SQLite and compression libraries.
analysis_version="$(cargo +1.95.0 metadata --locked --no-deps --format-version 1 \
  | jq -r '.packages[] | select(.name == "fensu-native") | .version')"
facts_package="${package_root}/package/fensu-facts-${analysis_version}"
native_package="${package_root}/package/fensu-native-${analysis_version}"
analysis_consumer="${consumer_root}/analysis-consumer"

CARGO_TARGET_DIR="${package_root}" cargo +1.95.0 package --locked -p fensu-facts -p fensu-native

cargo +stable new --quiet --bin "${analysis_consumer}"
cat >> "${analysis_consumer}/Cargo.toml" <<TOML
fensu-facts = { path = "${facts_package}", default-features = false }
fensu-native = { path = "${native_package}", default-features = false }

[patch.crates-io]
fensu-facts = { path = "${facts_package}" }
TOML
cat > "${analysis_consumer}/src/main.rs" <<'RUST'
use fensu_facts::ProgramHandle;
use fensu_native::rules::constants::PARAMETER_ANNOTATION_CODE;
use fensu_native::{evaluate_core_rules, NativeProjectPlane, NativeRuleContext};

fn main() {
    let program = ProgramHandle::parse_source("def total(items):\n    return items\n", 3, 12)
        .expect("the module parses");
    let faults = evaluate_core_rules(
        &program,
        &[PARAMETER_ANNOTATION_CODE.to_owned()],
        &NativeRuleContext::default(),
        &NativeProjectPlane::new(Vec::new(), Vec::new()),
    )
    .expect("the core rules run");
    assert_eq!(faults.len(), 1, "one unannotated parameter: {faults:?}");
    assert_eq!(faults[0].code, PARAMETER_ANNOTATION_CODE);
}
RUST
cargo +1.95.0 run --quiet --manifest-path "${analysis_consumer}/Cargo.toml"
if cargo +1.95.0 tree --manifest-path "${analysis_consumer}/Cargo.toml" -e normal \
  | grep -E '\b(pyo3|rusqlite|flate2) v'; then
  echo "the analysis crates pulled in Python, SQLite or compression dependencies" >&2
  exit 1
fi
