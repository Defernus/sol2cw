use sol2cw_lexer::{Lexer, Token};

use crate::{AstNode, AstResult};

#[derive(Debug, Clone, PartialEq)]
pub struct ContractKind {}

impl From<ContractKind> for AstNode {
    fn from(node: ContractKind) -> Self {
        AstNode::ContractKind(node)
    }
}

impl ContractKind {
    pub fn parse(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("parse ContractKind")
    }
}
