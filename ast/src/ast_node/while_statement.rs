use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {}

impl From<WhileStatement> for AstNode {
    fn from(node: WhileStatement) -> Self {
        AstNode::WhileStatement(node)
    }
}

impl WhileStatement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse WhileStatement")
    }
}
