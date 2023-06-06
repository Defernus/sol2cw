use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {}

impl From<Expression> for AstNode {
    fn from(node: Expression) -> Self {
        AstNode::Expression(node)
    }
}

impl Expression {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse Expression")
    }
}
