use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {}

impl From<Identifier> for AstNode {
    fn from(node: Identifier) -> Self {
        AstNode::Identifier(node)
    }
}

impl Identifier {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse Identifier")
    }
}
