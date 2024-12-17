#![expect(dead_code)]

use oxc::semantic::AstNode;

use crate::ctx::Ctx;

pub struct Subtest {
    pub name: &'static str,
    pub exec: &'static str,
}

pub trait Meta {
    fn name(&self) -> &'static str;
    fn target(&self) -> &'static str;
    fn category(&self) -> &'static str;
    fn spec(&self) -> &'static str;
    fn significance(&self) -> &'static str;
    fn mdn(&self) -> &'static str;
    fn exec(&self) -> &'static str;
    fn subtests(&self) -> Vec<Subtest>;
}

pub trait Feature: Meta {
    fn test(&self, _node: &AstNode<'_>, ctx: &mut Ctx);
}
