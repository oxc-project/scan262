use crate::features::Es2018ObjectRestSpreadProperties;
use crate::{ctx::Ctx, feature::Feature};
use oxc::ast::AstKind;
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es2018ObjectRestSpreadProperties {
    fn test(&self, node: &AstNode<'_>, semantic: &Semantic<'_>, ctx: &mut Ctx) {
        let span = match node.kind() {
            AstKind::ObjectPattern(obj) if obj.rest.is_some() => obj.rest.as_ref().unwrap().span,
            AstKind::SpreadElement(e)
                if matches!(
                    semantic.nodes().parent_kind(node.id()),
                    Some(AstKind::ObjectExpression(_))
                ) =>
            {
                e.span
            }
            _ => return,
        };
        ctx.diagnostic(self, span);
    }
}
