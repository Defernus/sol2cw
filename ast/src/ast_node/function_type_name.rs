use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionTypeName {}

impl From<FunctionTypeName> for AstNode {
    fn from(node: FunctionTypeName) -> Self {
        AstNode::FunctionTypeName(node)
    }
}

impl FunctionTypeName {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse FunctionTypeName")
    }
}
