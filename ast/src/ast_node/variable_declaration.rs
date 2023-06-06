use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {}

impl From<VariableDeclaration> for AstNode {
    fn from(node: VariableDeclaration) -> Self {
        AstNode::VariableDeclaration(node)
    }
}

impl VariableDeclaration {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse VariableDeclaration")
    }
}
