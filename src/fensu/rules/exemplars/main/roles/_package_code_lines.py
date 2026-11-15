"""Public custom equivalent of the runtime package code-line budget."""

import ast

from fensu import ExecutionOwner, Fault, RuleContext
from fensu.rules.exemplars._helpers.equivalent_rule import equivalent_rule
from fensu.rules.exemplars._helpers.non_file_rules import package_code_lines_impl


@equivalent_rule(
    core_code="FFR602",
    code="XCR602",
    slug="package-code-lines-equivalent",
    execution_owner=ExecutionOwner.SCOPE,
)
def package_code_lines_equivalent(*, module: ast.Module, ctx: RuleContext) -> list[Fault]:
    """Express FFR602 through public project, source, and threshold APIs."""

    return package_code_lines_impl(module=module, ctx=ctx)
