use crate::features::Es2020NullishCoalescingOperator;
use crate::{ctx::Ctx, feature::Feature};
use oxc::ast::ast::LogicalOperator;
use oxc::{
    ast::AstKind,
    semantic::{AstNode, Semantic},
};
impl Feature for Es2020NullishCoalescingOperator {
    fn test(&self, node: &AstNode<'_>, _semantic: &Semantic<'_>, ctx: &mut Ctx) {
        let AstKind::LogicalExpression(expr) = node.kind() else { return };
        if expr.operator != LogicalOperator::Coalesce {
            return;
        }
        ctx.diagnostic(self, expr.span);
    }
}
