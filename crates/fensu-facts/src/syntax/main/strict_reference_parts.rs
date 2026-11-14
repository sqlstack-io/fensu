//! Parse a strict dotted reference path.

use ruff_python_ast::Expr;

pub(crate) fn strict_reference_parts(expression: &Expr) -> Vec<String> {
    crate::syntax::_helpers::references::strict_reference_parts(expression)
}
