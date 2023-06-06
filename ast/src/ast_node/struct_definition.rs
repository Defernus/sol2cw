use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct StructDefinition {}

impl From<StructDefinition> for AstNode {
    fn from(node: StructDefinition) -> Self {
        AstNode::StructDefinition(node)
    }
}

impl StructDefinition {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse StructDefinition")
    }
}
