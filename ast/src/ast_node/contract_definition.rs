use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct ContractDefinition {}

impl From<ContractDefinition> for AstNode {
    fn from(node: ContractDefinition) -> Self {
        AstNode::ContractDefinition(node)
    }
}

impl ContractDefinition {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse ContractDefinition")
    }
}
