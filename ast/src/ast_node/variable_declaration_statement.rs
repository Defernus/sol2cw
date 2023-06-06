use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarationStatement {}

impl From<VariableDeclarationStatement> for AstNode {
    fn from(node: VariableDeclarationStatement) -> Self {
        AstNode::VariableDeclarationStatement(node)
    }
}

impl VariableDeclarationStatement {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse VariableDeclarationStatement")
    }
}
