use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue {}

impl From<EnumValue> for AstNode {
    fn from(node: EnumValue) -> Self {
        AstNode::EnumValue(node)
    }
}

impl EnumValue {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse EnumValue")
    }
}
