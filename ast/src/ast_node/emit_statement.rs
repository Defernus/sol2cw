use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct EmitStatement {}

impl From<EmitStatement> for AstNode {
    fn from(node: EmitStatement) -> Self {
        AstNode::EmitStatement(node)
    }
}

impl EmitStatement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse EmitStatement")
    }
}
