use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {}

impl From<Mapping> for AstNode {
    fn from(node: Mapping) -> Self {
        AstNode::Mapping(node)
    }
}

impl Mapping {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse Mapping")
    }
}
