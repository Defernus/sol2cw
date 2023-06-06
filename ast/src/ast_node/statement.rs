use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {}

impl From<Statement> for AstNode {
    fn from(node: Statement) -> Self {
        AstNode::Statement(node)
    }
}

impl Statement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse Statement")
    }
}
