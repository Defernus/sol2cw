use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList {}

impl From<ParameterList> for AstNode {
    fn from(node: ParameterList) -> Self {
        AstNode::ParameterList(node)
    }
}

impl ParameterList {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse ParameterList")
    }
}
