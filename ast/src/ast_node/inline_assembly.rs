use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct InlineAssembly {}

impl From<InlineAssembly> for AstNode {
    fn from(node: InlineAssembly) -> Self {
        AstNode::InlineAssembly(node)
    }
}

impl InlineAssembly {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse InlineAssembly")
    }
}
