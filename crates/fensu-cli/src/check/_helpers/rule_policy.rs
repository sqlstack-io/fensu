use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use fensu_policy::policy::errors::PolicyError;
use fensu_policy::policy::main::resolve_policy;
use fensu_policy::policy::main::validate_unique_implementations as generic_implementations;
use fensu_policy::policy::models::{FensuRuleCodeGrammar, PolicySelectors};
use fensu_policy::policy::types::PolicyTier;

use crate::catalogue::main::rule_catalogue::configured_rule_catalogue;
use crate::catalogue::main::rule_metadata::rule_metadata;
use crate::catalogue::main::validate_config_selectors::validate_config_selectors;
use crate::catalogue::models::RuleMetadata;
use crate::check::_helpers::policy::{path_matches, role};
use crate::check::models::RustThresholds;
use crate::configuration::main::configuration_directory::configuration_directory;
use crate::configuration::main::expand_path_pattern::expand_path_pattern;
use crate::models::{Config, ScopedSource, ThresholdUse};

const PATH_SEPARATOR: char = '/';
const RECURSIVE_GLOB: &str = "**";
const WILDCARD: char = '*';
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct PathSpecificity {
    literal_segments: usize,
    literal_characters: usize,
    globstars: Reverse<usize>,
    wildcards: Reverse<usize>,
}

pub(crate) fn display_codes_by_implementation(
    codes: &[String],
) -> Result<HashMap<String, String>, String> {
    let mut display: HashMap<String, String> = HashMap::new();
    for code in codes {
        let metadata =
            rule_metadata(code)?.ok_or_else(|| format!("Unknown native rule code: {code}"))?;
        let implementation = metadata.alias_of.as_ref().unwrap_or(code);
        if let Some(previous) = display.insert(implementation.clone(), code.clone()) {
            return Err(format!(
                "Rules {previous} and {code} select the same native implementation {implementation}; select only one identity."
            ));
        }
    }
    Ok(display)
}

pub(crate) fn selected_rules(
    config: &Config,
    select: &[String],
    ignore: &[String],
) -> Result<Vec<&'static RuleMetadata>, String> {
    let catalogue = configured_rule_catalogue(&config.rule_packs)?;
    let selection = resolve_policy::resolve_policy(
        &catalogue,
        &config.analyzer,
        &PolicySelectors {
            select: select.to_vec(),
            warn: Vec::new(),
            ignore: ignore.to_vec(),
        },
        &FensuRuleCodeGrammar,
    )
    .map_err(format_policy_error)?;
    let rules = selection.blocking;
    validate_unique_implementations(&rules)?;
    Ok(rules)
}

pub(crate) fn placement_rules_selected(config: &Config, warnings: bool) -> Result<bool, String> {
    let select = config
        .select
        .iter()
        .chain(config.warn.iter().filter(|_| warnings))
        .filter(|code| !code.starts_with('X'))
        .cloned()
        .collect::<Vec<_>>();
    let ignore = config
        .ignore
        .iter()
        .filter(|code| !code.starts_with('X'))
        .cloned()
        .collect::<Vec<_>>();
    Ok(selected_rules(config, &select, &ignore)?
        .iter()
        .any(|rule| {
            matches!(
                rule.implementation_code.as_deref(),
                Some("RSR311" | "RSL106")
            )
        }))
}

pub(crate) fn validate_config_tiers(config: &Config) -> Result<(), String> {
    let configured_catalogue = configured_rule_catalogue(&config.rule_packs)?;
    let catalogue = configured_catalogue
        .iter()
        .copied()
        .filter(|rule| applicable(rule, config))
        .collect::<Vec<_>>();
    let custom_hosted = config.analyzer != crate::analyzer::AnalyzerId::Python
        && (!config.rule_paths.is_empty() || !config.rule_modules.is_empty());
    if !custom_hosted {
        validate_config_selectors(config, &catalogue, &configured_catalogue)?;
    }
    let selectors = |values: &[String]| {
        values
            .iter()
            .filter(|value| !custom_hosted || !value.starts_with('X'))
            .cloned()
            .collect::<Vec<_>>()
    };
    let _ = resolve_policy::resolve_policy(
        &configured_catalogue,
        &config.analyzer,
        &PolicySelectors {
            select: selectors(&config.select),
            warn: selectors(&config.warn),
            ignore: selectors(&config.ignore),
        },
        &FensuRuleCodeGrammar,
    )
    .map_err(format_policy_error)?;
    Ok(())
}

pub(crate) fn required_thresholds(codes: &[String]) -> Result<HashSet<&'static str>, String> {
    let mut names: HashSet<&'static str> = HashSet::new();
    for code in codes {
        if let Some(metadata) = rule_metadata(code)? {
            names.extend(metadata.thresholds.iter().map(String::as_str));
        }
    }
    Ok(names)
}

pub(crate) fn resolved_thresholds(
    source: &ScopedSource,
    config: &Config,
    codes: &[String],
) -> Result<(HashMap<String, u32>, Vec<ThresholdUse>), String> {
    let mut values = config.thresholds.clone();
    if let Some(role) = role(source).and_then(|role| config.role_thresholds.get(&role)) {
        values.extend(role.clone());
    }
    let mut uses: Vec<ThresholdUse> = Vec::new();
    for name in required_thresholds(codes)? {
        let Some(use_) =
            path_threshold_use(config, &source.target_path, &source.repository_path, name)?
        else {
            continue;
        };
        values.insert(name.to_owned(), use_.effective_value);
        uses.push(use_);
    }
    Ok((values, uses))
}

impl fensu_rust::types::ThresholdLookup for RustThresholds<'_> {
    /// Resolve a threshold, recording an applied override only for selected rules.
    fn threshold(&self, name: &str, path: &str) -> Option<usize> {
        rust_threshold(self, name, path)
    }
}

fn rust_threshold(thresholds: &RustThresholds<'_>, name: &str, path: &str) -> Option<usize> {
    let RustThresholds {
        config,
        project_root,
        required,
        uses,
    } = thresholds;
    let configured = config.thresholds.get(name).map(|value| *value as usize);
    if !required.contains(name) {
        return configured;
    }
    let directory = configuration_directory(project_root, &config.target_root);
    let repository_path = project_root
        .join(path)
        .strip_prefix(&directory)
        .map_or_else(
            |_| path.to_owned(),
            |relative| relative.to_string_lossy().replace('\\', "/"),
        );
    match path_threshold_use(config, path, &repository_path, name) {
        Ok(Some(use_)) => {
            let value = use_.effective_value as usize;
            if let Ok(recorded) = uses.borrow_mut().as_mut() {
                recorded.push(use_);
            }
            Some(value)
        }
        Ok(None) => configured,
        Err(error) => {
            *uses.borrow_mut() = Err(error);
            configured
        }
    }
}

fn path_threshold_use(
    config: &Config,
    target_path: &str,
    repository_path: &str,
    name: &str,
) -> Result<Option<ThresholdUse>, String> {
    let mut winner = None;
    for (order, override_) in config.threshold_overrides.iter().enumerate() {
        let Some(value) = override_.thresholds.get(name) else {
            continue;
        };
        for (pattern_order, pattern) in override_.paths.iter().enumerate() {
            let Some(specificity) = matching_path_specificity(target_path, pattern)? else {
                continue;
            };
            let rank = (specificity, order, pattern_order);
            if winner
                .as_ref()
                .is_none_or(|(current, _, _, _, _)| rank >= *current)
            {
                winner = Some((rank, order, pattern, &override_.reason, *value));
            }
        }
    }
    Ok(
        winner.map(|(_, order, pattern, reason, value)| ThresholdUse {
            repository_path: repository_path.to_owned(),
            threshold: name.to_owned(),
            override_order: order,
            matched_pattern: pattern.clone(),
            reason: reason.clone(),
            effective_value: value,
        }),
    )
}

pub(crate) fn applicable(rule: &RuleMetadata, config: &Config) -> bool {
    rule.analyzers.contains(&config.analyzer)
}

fn path_specificity(pattern: &str) -> PathSpecificity {
    let segments = pattern.split(PATH_SEPARATOR).collect::<Vec<_>>();
    let literal_segments = segments
        .iter()
        .filter(|segment| !segment.contains(WILDCARD))
        .count();
    let literal_characters = pattern
        .chars()
        .filter(|character| !matches!(character, &WILDCARD | &PATH_SEPARATOR))
        .count();
    let globstars = segments
        .iter()
        .filter(|segment| **segment == RECURSIVE_GLOB)
        .count();
    PathSpecificity {
        literal_segments,
        literal_characters,
        globstars: Reverse(globstars),
        wildcards: Reverse(wildcard_count(&segments)),
    }
}

fn matching_path_specificity(path: &str, pattern: &str) -> Result<Option<PathSpecificity>, String> {
    Ok(expand_path_pattern(pattern)?
        .into_iter()
        .filter_map(|expanded| path_matches(path, &expanded).then(|| path_specificity(&expanded)))
        .max())
}

pub(crate) fn validate_unique_implementations(rules: &[&RuleMetadata]) -> Result<(), String> {
    generic_implementations::validate_unique_implementations(rules).map_err(format_policy_error)
}

fn format_policy_error(error: PolicyError) -> String {
    match error {
        PolicyError::TierConflict {
            code,
            first: PolicyTier::Blocking,
            second: PolicyTier::Warning,
        } => format!("Rule {code} cannot be configured as both blocking and warning."),
        PolicyError::TierConflict {
            code,
            first: PolicyTier::Warning,
            second: PolicyTier::Ignored,
        } => format!("Rule {code} cannot be configured as both warning and ignored."),
        PolicyError::DuplicateImplementation {
            first_code,
            second_code,
            implementation_code,
        } => format!(
            "Rules {first_code} and {second_code} select the same native implementation {implementation_code}; select only one identity."
        ),
        other => format!("Invalid rule policy: {other}"),
    }
}

fn wildcard_count(segments: &[&str]) -> usize {
    let mut count = 0;
    for segment in segments {
        if *segment == RECURSIVE_GLOB {
            continue;
        }
        count += segment
            .chars()
            .filter(|character| *character == WILDCARD)
            .count();
    }
    count
}
