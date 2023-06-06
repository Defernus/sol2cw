use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {}

impl From<ForStatement> for AstNode {
    fn from(node: ForStatement) -> Self {
        AstNode::ForStatement(node)
    }
}

impl ForStatement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse ForStatement")
    }
}
