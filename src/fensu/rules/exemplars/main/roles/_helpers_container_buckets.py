"""Public custom equivalent of the helpers container bucket budget."""

import ast

from fensu import ExecutionOwner, Fault, RuleContext
from fensu.rules.exemplars._helpers.equivalent_rule import equivalent_rule
from fensu.rules.exemplars._helpers.non_file_rules import helpers_container_buckets_impl


@equivalent_rule(
    core_code="FFR312",
    code="XCR312",
    slug="helpers-container-buckets-equivalent",
    execution_owner=ExecutionOwner.PACKAGE,
)
def helpers_container_buckets_equivalent(*, module: ast.Module, ctx: RuleContext) -> list[Fault]:
    """Express FFR312 through public project and threshold APIs."""

    return helpers_container_buckets_impl(module=module, ctx=ctx)
