use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierPath {}

impl From<IdentifierPath> for AstNode {
    fn from(node: IdentifierPath) -> Self {
        AstNode::IdentifierPath(node)
    }
}

impl IdentifierPath {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse IdentifierPath")
    }
}
