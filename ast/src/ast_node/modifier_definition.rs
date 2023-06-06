use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct ModifierDefinition {}

impl From<ModifierDefinition> for AstNode {
    fn from(node: ModifierDefinition) -> Self {
        AstNode::ModifierDefinition(node)
    }
}

impl ModifierDefinition {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse ModifierDefinition")
    }
}
