//! Fensu's Python rule engine over `fensu-facts` programs; use it with `default-features = false`.
#![forbid(unsafe_code)]

#[cfg(feature = "python")]
pub(crate) mod cache;
#[cfg(feature = "python")]
pub(crate) mod extension;
pub mod rules;

pub use crate::rules::main::evaluate_core_rules::evaluate_core_rules;
pub use crate::rules::main::evaluate_dead_code::evaluate_dead_code;
pub use crate::rules::main::generated_policy_owners::generated_policy_owners;
pub use crate::rules::main::parse_dead_code::parse_dead_code;
pub use crate::rules::main::plan_core_rule_queries::plan_core_rule_queries;
pub use crate::rules::main::plan_execution_owners::plan_execution_owners;
pub use crate::rules::models::{
    CustomRegistration, NativeDeadCodeContext, NativeExecutionPlan, NativeExecutionRule,
    NativeExecutionTarget, NativeFaultRow, NativeProjectModule, NativeProjectPlane,
    NativeProjectQuery, NativeRuleContext,
};
pub use crate::rules::types::NativeDeadCodeSettings;
