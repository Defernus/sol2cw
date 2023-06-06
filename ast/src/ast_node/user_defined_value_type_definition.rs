use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedValueTypeDefinition {}

impl From<UserDefinedValueTypeDefinition> for AstNode {
    fn from(node: UserDefinedValueTypeDefinition) -> Self {
        AstNode::UserDefinedValueTypeDefinition(node)
    }
}

impl UserDefinedValueTypeDefinition {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse UserDefinedValueTypeDefinition")
    }
}
