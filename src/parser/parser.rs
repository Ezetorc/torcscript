use crate::{
    abstract_syntax_tree::{expression::Expression, operator::Operator, statement::Statement},
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

    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>, LangError> {
        let mut abstract_syntax_tree: Vec<Statement> = Vec::new();
        let mut parser: Parser = Parser::new(tokens);

        while !parser.is_at_end() {
            let token: Token = parser.get_current_token();

            let result: Result<Statement, LangError> = match token {
                Token::Keyword(Keyword::Variable) => parser.handle_variable_declaration(),
                Token::Keyword(Keyword::Print) => parser.handle_print(),
                Token::Identifier(identifier) => parser.handle_variable_assignation(identifier),
                Token::EndOfFile => break,
                Token::EndOfLine => {
                    parser.advance();
                    continue;
                }
                _ => {
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

    pub fn parse_expression(&mut self) -> Expression {
        self.parse_term()
    }

    pub fn parse_term(&mut self) -> Expression {
        let mut expression: Expression = self.parse_factor();

        while let Token::Operator(operator) = self.get_current_token() {
            if operator.is_additive() {
                self.advance();
                let right: Expression = self.parse_factor();

                expression = Expression::Binary {
                    left: Box::new(expression),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        expression
    }

    fn parse_factor(&mut self) -> Expression {
        let mut expression: Expression = self.parse_unary();

        while let Token::Operator(operator) = self.get_current_token() {
            if operator.is_multiplicative() {
                self.advance();
                let right: Expression = self.parse_unary();

                expression = Expression::Binary {
                    left: Box::new(expression),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        expression
    }

    fn parse_unary(&mut self) -> Expression {
        match self.get_current_token() {
            Token::Operator(Operator::Substraction) => {
                self.advance();

                let expression: Expression = self.parse_unary();

                Expression::Unary {
                    operator: Operator::Substraction,
                    expression: Box::new(expression),
                }
            }

            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Expression {
        match self.advance() {
            Token::Literal(literal) => Expression::Literal(literal),

            Token::Identifier(name) => Expression::Identifier(name),

            _ => panic!("Unexpected token"),
        }
    }

    pub fn advance(&mut self) -> Token {
        let token: Token = self.get_current_token();

        if !self.is_at_end() {
            self.current += 1;
        }

        token
    }

    pub fn advance_expecting(&mut self, expected_token: Token) -> Result<Token, LangError> {
        let token: Token = self.get_current_token();

        if token != expected_token {
            return Err(LangError::Parser(ParserError::InvalidSyntax(format!(
                "Expected {:?}, found {:?}",
                expected_token, token
            ))));
        }

        if !self.is_at_end() {
            self.current += 1;
        }

        Ok(token)
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
