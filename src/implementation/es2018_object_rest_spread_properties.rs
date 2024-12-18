use crate::features::Es2018ObjectRestSpreadProperties;
use crate::{ctx::Ctx, feature::Feature};
use oxc::ast::ast::ObjectPropertyKind;
use oxc::ast::AstKind;
use oxc::semantic::AstNode;
impl Feature for Es2018ObjectRestSpreadProperties {
    fn test(&self, node: &AstNode<'_>, ctx: &mut Ctx) {
        match node.kind() {
            AstKind::ObjectPattern(obj) if obj.rest.is_some() => {
                ctx.diagnostic(self, obj.rest.as_ref().unwrap().span);
            }
            AstKind::ObjectExpression(expr) => {
                for obj in &expr.properties {
                    if let ObjectPropertyKind::SpreadProperty(s) = obj {
                        ctx.diagnostic(self, s.span);
                    }
                }
            }
            _ => return,
        }
    }
}
