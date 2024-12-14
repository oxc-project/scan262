use oxc::{semantic::AstNode, span::Span};

pub trait Feature {
    fn name(&self) -> &'static str;

    fn test(&self, _node: &AstNode<'_>) -> Option<Span>;
}
