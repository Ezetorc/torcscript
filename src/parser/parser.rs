use crate::{
    abstract_syntax_tree::statement::Statement,
    errors::{lang_error::LangError, parser_error::ParserError},
    lexer::{keyword::Keyword, token::Token},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn evaluate(tokens: Vec<Token>) -> Result<Vec<Statement>, LangError> {
        let mut abstract_syntax_tree: Vec<Statement> = Vec::new();
        let mut parser: Parser = Parser::new(tokens);

        while !parser.is_at_end() {
            let token: Token = parser.get_current_token();

            let result: Result<Statement, LangError> = match token {
                Token::Keyword(Keyword::Print) => parser.handle_print(),

                Token::EndOfLine => {
                    parser.advance();
                    continue;
                }

                Token::EndOfFile => break,

                _ => {
                    println!("token {:?}", token);
                    return Err(LangError::from(ParserError::NotImplemented(
                        "Token parsement not yet implemented".to_string(),
                    )));
                }
            };

            match result {
                Ok(statement) => abstract_syntax_tree.push(statement),
                Err(error) => return Err(error),
            }
        }

        Ok(abstract_syntax_tree)
    }

    pub fn advance(&mut self) -> Token {
        let token = self.get_current_token();

        if !self.is_at_end() {
            self.current += 1;
        }

        token
    }

    pub fn get_current_token(&self) -> Token {
        self.tokens
            .get(self.current)
            .cloned()
            .unwrap_or(Token::EndOfFile)
    }

    pub fn is_at_end(&self) -> bool {
        self.get_current_token() == Token::EndOfFile
    }
}
