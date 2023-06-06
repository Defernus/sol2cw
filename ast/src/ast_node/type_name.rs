use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct TypeName {}

impl From<TypeName> for AstNode {
    fn from(node: TypeName) -> Self {
        AstNode::TypeName(node)
    }
}

impl TypeName {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse TypeName")
    }
}
