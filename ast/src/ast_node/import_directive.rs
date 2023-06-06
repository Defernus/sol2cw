use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDirective {}

impl From<ImportDirective> for AstNode {
    fn from(node: ImportDirective) -> Self {
        AstNode::ImportDirective(node)
    }
}

impl ImportDirective {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse ImportDirective")
    }
}
