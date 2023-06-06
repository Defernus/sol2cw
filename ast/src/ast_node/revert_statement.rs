use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct RevertStatement {}

impl From<RevertStatement> for AstNode {
    fn from(node: RevertStatement) -> Self {
        AstNode::RevertStatement(node)
    }
}

impl RevertStatement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse RevertStatement")
    }
}
