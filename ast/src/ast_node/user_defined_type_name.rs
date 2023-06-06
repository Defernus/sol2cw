use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedTypeName {}

impl From<UserDefinedTypeName> for AstNode {
    fn from(node: UserDefinedTypeName) -> Self {
        AstNode::UserDefinedTypeName(node)
    }
}

impl UserDefinedTypeName {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse UserDefinedTypeName")
    }
}
