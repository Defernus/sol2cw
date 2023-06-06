use sol2cw_lexer::{Lexer, Token};

use crate::{ast_node::AstNode, AstResult};

pub struct AstParser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> AstParser<'a> {
    pub fn new(sources: &'a str) -> Self {
        Self {
            lexer: Lexer::new(sources),
        }
    }

    pub fn parse(mut self) -> AstResult<Vec<AstNode>> {
        let mut result = Vec::new();

        loop {
            let token = self.lexer.next_token()?;
            match token {
                Token::Eof => break,
                token => {
                    let ast_node = AstNode::from_tokens(token, &mut self.lexer)?;
                    result.push(ast_node);
                }
            }
        }

        Ok(result)
    }
}
