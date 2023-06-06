use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {}

impl From<IfStatement> for AstNode {
    fn from(node: IfStatement) -> Self {
        AstNode::IfStatement(node)
    }
}

impl IfStatement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse IfStatement")
    }
}
