use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct OverrideSpecifier {}

impl From<OverrideSpecifier> for AstNode {
    fn from(node: OverrideSpecifier) -> Self {
        AstNode::OverrideSpecifier(node)
    }
}

impl OverrideSpecifier {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse OverrideSpecifier")
    }
}
