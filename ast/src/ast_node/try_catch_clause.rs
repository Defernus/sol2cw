use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct TryCatchClause {}

impl From<TryCatchClause> for AstNode {
    fn from(node: TryCatchClause) -> Self {
        AstNode::TryCatchClause(node)
    }
}

impl TryCatchClause {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse TryCatchClause")
    }
}
