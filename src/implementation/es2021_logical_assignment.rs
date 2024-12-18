use oxc::{
    ast::AstKind,
    semantic::{AstNode, Semantic},
};

use crate::{ctx::Ctx, feature::Feature, features::Es2021LogicalAssignment};
impl Feature for Es2021LogicalAssignment {
    fn test(&self, node: &AstNode<'_>, _semantic: &Semantic<'_>, ctx: &mut Ctx) {
        let AstKind::AssignmentExpression(expr) = node.kind() else { return };
        if !expr.operator.is_logical() {
            return;
        }
        ctx.diagnostic(self, expr.span);
    }
}
