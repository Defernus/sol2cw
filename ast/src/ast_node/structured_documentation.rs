use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredDocumentation {}

impl From<StructuredDocumentation> for AstNode {
    fn from(node: StructuredDocumentation) -> Self {
        AstNode::StructuredDocumentation(node)
    }
}

impl StructuredDocumentation {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse StructuredDocumentation")
    }
}
