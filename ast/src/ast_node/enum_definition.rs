use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDefinition {}

impl From<EnumDefinition> for AstNode {
    fn from(node: EnumDefinition) -> Self {
        AstNode::EnumDefinition(node)
    }
}

impl EnumDefinition {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse EnumDefinition")
    }
}
