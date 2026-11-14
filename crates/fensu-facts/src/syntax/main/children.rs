//! Collect the direct children of one syntax node.

use crate::syntax::_helpers::children::ChildOutput;
use crate::syntax::types::ShapeNode;

pub(crate) fn children<'a>(node: &ShapeNode<'a>, out: &mut Vec<ShapeNode<'a>>) {
    ChildOutput { out }.collect(node);
}
