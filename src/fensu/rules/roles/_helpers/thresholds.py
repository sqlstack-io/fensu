"""Threshold ownership for role rules."""

from __future__ import annotations

from fensu.rules.authoring.types import Threshold
from fensu.rules.roles.types import RoleCode


def get_role_rule_thresholds(code: RoleCode) -> tuple[Threshold, ...]:
    """Return every configurable threshold consumed by one role rule."""

    thresholds: dict[RoleCode, tuple[Threshold, ...]] = {
        RoleCode.HELPERS_PACKAGE_LAYOUT: (
            Threshold.MAX_HELPERS_CONTAINER_MODULES,
            Threshold.MAX_ROLE_DEPTH,
        ),
        RoleCode.MAIN_PACKAGE_LAYOUT: (
            Threshold.MAX_MAIN_CONTAINER_MODULES,
            Threshold.MAX_ROLE_DEPTH,
        ),
        RoleCode.HELPERS_CONTAINER_BUCKETS: (Threshold.MAX_HELPERS_CONTAINER_BUCKETS,),
        RoleCode.SHARED_DOMAIN_PREFIX: (Threshold.MIN_SHARED_DOMAIN_PREFIX_PACKAGES,),
        RoleCode.SOURCE_FILE_LINE_COUNT: (Threshold.MAX_FILE_LINES,),
        RoleCode.TOP_LEVEL_PACKAGE_CODE_LINES: (Threshold.MAX_CRATE_CODE_LINES,),
        RoleCode.TOOLING_ENTRYPOINT_LINE_COUNT: (Threshold.MAX_SCRIPT_ENTRYPOINT_LINES,),
        RoleCode.CUSTOM_RULE_TEST_COVERAGE: (Threshold.MIN_CUSTOM_RULE_TEST_CASES,),
    }
    return thresholds.get(code, ())
