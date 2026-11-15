//! Dependency plans for non-file native role layout rules.

use std::path::Path;

use crate::rules::_helpers::role_project_layout_paths::{
    code_lines_query, direct_modules, directory_entries, domain_dir, domain_roots, file_name,
    grouping_roots, leaf_dir, main_entries, mixed_domain, named_subdomains, ownership_roots,
    prefix_candidates, prefix_groups, python_anchor, recursive_python, repository_path, role_name,
    role_package, scope_root, HELPERS, INIT_FILE, PYTHON_CACHE, ROOT_SCOPE,
};
use crate::rules::constants::{
    HELPERS_CONTAINER_BUCKETS_CODE, HELPERS_PACKAGE_LAYOUT_CODE, LEAF_MAIN_BOUNDARY_CODE,
    MAIN_PACKAGE_LAYOUT_CODE, PACKAGE_CODE_LINES_CODE, SHARED_DOMAIN_PREFIX_CODE,
    TOP_LEVEL_DOMAIN_SHAPE_CODE,
};
use crate::rules::models::{NativeProjectQuery, NativeRuleContext};

const MAIN_ROLE: &str = "main";
const MINIMUM_SHARED_PREFIX_THRESHOLD: &str = "min_shared_domain_prefix_packages";
const PYTHON_GLOB: &str = "*.py";

pub(crate) fn project_layout_queries(
    code: &str,
    context: &NativeRuleContext,
) -> Option<Vec<NativeProjectQuery>> {
    let mut queries: Vec<NativeProjectQuery> = Vec::new();
    match code {
        HELPERS_PACKAGE_LAYOUT_CODE | MAIN_PACKAGE_LAYOUT_CODE => {
            let role = if code == HELPERS_PACKAGE_LAYOUT_CODE {
                HELPERS
            } else {
                MAIN_ROLE
            };
            let Some(package) = role_package(context, role) else {
                return Some(queries);
            };
            let container = repository_path(context)
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_default();
            if context.relative_parts.last().map(String::as_str) != Some(INIT_FILE) {
                queries.push(query("exists", &container.join(INIT_FILE), ""));
            }
            queries.push(glob_query(&container, false));
            if !direct_modules(&container).is_empty() {
                queries.push(glob_query(&container, true));
            }
            if container != package {
                let mut ancestor = container.parent();
                while let Some(path) = ancestor {
                    if path == package {
                        break;
                    }
                    queries.push(query("python_anchor", path, ""));
                    if python_anchor(path).as_deref() != Some(repository_path(context).as_path()) {
                        break;
                    }
                    ancestor = path.parent();
                }
            }
        }
        HELPERS_CONTAINER_BUCKETS_CODE => queries = bucket_queries(context),
        PACKAGE_CODE_LINES_CODE => queries = code_line_queries(context),
        TOP_LEVEL_DOMAIN_SHAPE_CODE => {
            queries = append_grouping_discovery_queries(queries, context);
            for group in grouping_roots(context) {
                queries.push(query("directory_entries", &group, ""));
            }
            queries = append_domain_discovery_queries(queries, context);
            for domain in domain_roots(context) {
                queries.push(query("directory_entries", &domain, ""));
                queries = append_subdomain_queries(queries, &domain);
                if mixed_domain(&domain) {
                    queries.push(query("is_file", &domain.join(INIT_FILE), ""));
                    if !domain.join(INIT_FILE).is_file() {
                        queries.push(glob_query(&domain, true));
                    }
                }
            }
        }
        SHARED_DOMAIN_PREFIX_CODE => {
            if context
                .thresholds
                .get(MINIMUM_SHARED_PREFIX_THRESHOLD)
                .copied()
                .unwrap_or_default()
                == 0
            {
                return Some(queries);
            }
            for root in ownership_roots(context) {
                queries.push(query("is_file", &root.join(INIT_FILE), ""));
                if !root.join(INIT_FILE).is_file() {
                    queries.push(glob_query(&root, true));
                }
                queries.push(query("directory_entries", &root, ""));
                for entry in prefix_candidates(&root) {
                    queries.push(query("is_dir", &entry, ""));
                    if entry.is_dir() {
                        queries.push(glob_query(&entry, true));
                    }
                }
                for (prefix, names) in prefix_groups(&root) {
                    let minimum = context.thresholds[MINIMUM_SHARED_PREFIX_THRESHOLD] as usize;
                    if names.len() >= minimum {
                        queries.push(query("is_dir", &root.join(prefix), ""));
                    }
                }
            }
        }
        LEAF_MAIN_BOUNDARY_CODE => {
            let Some(leaf) = leaf_dir(context) else {
                return Some(queries);
            };
            let Some(domain) = domain_dir(context) else {
                return Some(queries);
            };
            if leaf == domain {
                queries.push(query("directory_entries", &domain, ""));
                queries = append_subdomain_queries(queries, &domain);
                if !named_subdomains(&domain).is_empty() {
                    return Some(queries);
                }
            }
            queries.push(glob_query(&leaf.join(MAIN_ROLE), true));
            if main_entries(&leaf).is_empty() {
                queries.push(query("python_anchor", &leaf, ""));
            }
        }
        _ => return None,
    }
    Some(queries)
}

fn bucket_queries(context: &NativeRuleContext) -> Vec<NativeProjectQuery> {
    let mut queries: Vec<NativeProjectQuery> = Vec::new();
    let Some(container) = role_package(context, HELPERS) else {
        return queries;
    };
    queries.push(query("python_anchor", &container, ""));
    if python_anchor(&container).as_deref() != Some(repository_path(context).as_path()) {
        return queries;
    }
    queries.push(query("directory_entries", &container, ""));
    for entry in directory_entries(&container) {
        if file_name(&entry) == PYTHON_CACHE {
            continue;
        }
        queries.push(query("is_dir", &entry, ""));
        if entry.is_dir() {
            queries.push(glob_query(&entry, true));
        }
    }
    queries
}

fn code_line_queries(context: &NativeRuleContext) -> Vec<NativeProjectQuery> {
    if context.scope != ROOT_SCOPE {
        return Vec::new();
    }
    let root = scope_root(context);
    let mut queries = vec![glob_query(&root, true)];
    for path in recursive_python(&root) {
        queries.push(code_lines_query(context, &path));
    }
    queries
}

fn append_grouping_discovery_queries(
    mut queries: Vec<NativeProjectQuery>,
    context: &NativeRuleContext,
) -> Vec<NativeProjectQuery> {
    for root in grouping_roots(context) {
        queries.push(query("directory_entries", &root, ""));
        for entry in directory_entries(&root) {
            if role_name(&entry) || file_name(&entry) == PYTHON_CACHE {
                continue;
            }
            queries.push(query("is_dir", &entry, ""));
            if entry.is_dir() {
                queries.push(glob_query(&entry, true));
            }
        }
    }
    queries
}

fn append_domain_discovery_queries(
    mut queries: Vec<NativeProjectQuery>,
    context: &NativeRuleContext,
) -> Vec<NativeProjectQuery> {
    for root in ownership_roots(context) {
        queries.push(query("directory_entries", &root, ""));
        for entry in directory_entries(&root) {
            if role_name(&entry) || file_name(&entry) == PYTHON_CACHE {
                continue;
            }
            queries.push(query("is_dir", &entry, ""));
            if entry.is_dir() {
                queries.push(glob_query(&entry, true));
            }
        }
    }
    queries
}

fn append_subdomain_queries(
    mut queries: Vec<NativeProjectQuery>,
    domain: &Path,
) -> Vec<NativeProjectQuery> {
    for entry in directory_entries(domain) {
        if role_name(&entry) || file_name(&entry) == PYTHON_CACHE {
            continue;
        }
        queries.push(query("is_dir", &entry, ""));
        if entry.is_dir() {
            queries.push(glob_query(&entry, true));
        }
    }
    queries
}

fn query(kind: &str, path: &Path, argument: &str) -> NativeProjectQuery {
    NativeProjectQuery {
        kind: kind.to_owned(),
        path: path.to_string_lossy().replace('\\', "/"),
        argument: argument.to_owned(),
    }
}

fn glob_query(path: &Path, recursive: bool) -> NativeProjectQuery {
    query("glob", path, &format!("{PYTHON_GLOB}\0{recursive}"))
}
