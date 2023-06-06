use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct InheritanceSpecifier {}

impl From<InheritanceSpecifier> for AstNode {
    fn from(node: InheritanceSpecifier) -> Self {
        AstNode::InheritanceSpecifier(node)
    }
}

impl InheritanceSpecifier {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse InheritanceSpecifier")
    }
}
