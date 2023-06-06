use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct PragmaDirective {}

impl From<PragmaDirective> for AstNode {
    fn from(node: PragmaDirective) -> Self {
        AstNode::PragmaDirective(node)
    }
}

impl PragmaDirective {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse PragmaDirective")
    }
}
