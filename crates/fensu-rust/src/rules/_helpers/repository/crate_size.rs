//! Bound the non-test code lines one workspace crate holds.

use std::collections::BTreeSet;
use std::path;

use syn::spanned::Spanned;

use crate::constants;
use crate::models;
use crate::rules::_helpers::functions::comments;
use crate::rules::_helpers::sources::scanning;
use crate::types::ThresholdLookup;

const CFG_ATTRIBUTE: &str = "cfg";
const TEST_CONFIGURATION: &str = "test";
const THOUSANDS_GROUP: usize = 3;

pub(crate) fn check(
    repo_root: &path::Path,
    workspace_crate: &models::WorkspaceCrate,
    thresholds: &models::ThresholdConfig,
    lookup: &dyn ThresholdLookup,
) -> Option<models::Violation> {
    let manifest = scanning::relative_display(
        repo_root,
        &workspace_crate
            .directory
            .join(constants::CARGO_MANIFEST_FILE),
    );
    let limit = lookup
        .threshold(constants::CRATE_CODE_LINES_THRESHOLD, &manifest)
        .unwrap_or(thresholds.max_crate_code_lines);
    let total: usize = workspace_crate
        .files
        .iter()
        .filter(|file| scanning::file_matches_target(file, &workspace_crate.targets, false))
        .filter(|file| !scanning::is_inline_test_file(file))
        .map(code_line_count)
        .sum();
    (total > limit).then(|| {
        models::Violation::new(models::ViolationRequest {
            code: constants::CRATE_CODE_LINES_CODE,
            path: path::Path::new(&manifest),
            line: None,
            message: format!(
                "crate holds {} code lines (limit {}); split it into crates by area.",
                grouped(total),
                grouped(limit)
            ),
            remediation: "split the crate into crates by area; blank lines, comments, doc comments, tests/ targets, inline test trees, and #[cfg(test)] items do not count",
        })
    })
}

fn code_line_count(file: &models::SourceFile) -> usize {
    let mut lines = comments::code_lines(&file.source);
    let Some(syntax) = file.syntax.file.as_ref() else {
        return lines.len();
    };
    if syntax.attrs.iter().any(is_test_configuration) {
        return 0;
    }
    let excluded: BTreeSet<usize> = syntax
        .items
        .iter()
        .filter(|item| item_attributes(item).iter().any(is_test_configuration))
        .flat_map(|item| item.span().start().line..=item.span().end().line)
        .collect();
    lines.retain(|line| !excluded.contains(line));
    lines.len()
}

fn item_attributes(item: &syn::Item) -> &[syn::Attribute] {
    match item {
        syn::Item::Const(item) => &item.attrs,
        syn::Item::Enum(item) => &item.attrs,
        syn::Item::ExternCrate(item) => &item.attrs,
        syn::Item::Fn(item) => &item.attrs,
        syn::Item::ForeignMod(item) => &item.attrs,
        syn::Item::Impl(item) => &item.attrs,
        syn::Item::Macro(item) => &item.attrs,
        syn::Item::Mod(item) => &item.attrs,
        syn::Item::Static(item) => &item.attrs,
        syn::Item::Struct(item) => &item.attrs,
        syn::Item::Trait(item) => &item.attrs,
        syn::Item::TraitAlias(item) => &item.attrs,
        syn::Item::Type(item) => &item.attrs,
        syn::Item::Union(item) => &item.attrs,
        syn::Item::Use(item) => &item.attrs,
        _ => &[],
    }
}

fn is_test_configuration(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident(CFG_ATTRIBUTE)
        && matches!(
            &attribute.meta,
            syn::Meta::List(list) if list.tokens.to_string() == TEST_CONFIGURATION
        )
}

fn grouped(value: usize) -> String {
    let digits = value.to_string();
    let mut text = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(THOUSANDS_GROUP) {
            text.push(',');
        }
        text.push(digit);
    }
    text
}
