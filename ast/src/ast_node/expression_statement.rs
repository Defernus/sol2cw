use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {}

impl From<ExpressionStatement> for AstNode {
    fn from(node: ExpressionStatement) -> Self {
        AstNode::ExpressionStatement(node)
    }
}

impl ExpressionStatement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse ExpressionStatement")
    }
}
