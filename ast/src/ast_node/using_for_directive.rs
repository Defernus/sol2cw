use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct UsingForDirective {}

impl From<UsingForDirective> for AstNode {
    fn from(node: UsingForDirective) -> Self {
        AstNode::UsingForDirective(node)
    }
}

impl UsingForDirective {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse UsingForDirective")
    }
}
