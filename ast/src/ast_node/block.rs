use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {}

impl From<Block> for AstNode {
    fn from(node: Block) -> Self {
        AstNode::Block(node)
    }
}

impl Block {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse Block")
    }
}
