use crate::{
    abstract_syntax_tree::{expression::Expression, operator::Operator, statement::Statement},
    errors::{lang_error::LangError, parser_error::ParserError},
    lexer::{keyword::Keyword, side::Side, token::Token},
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
        let mut parser: Parser = Parser::new(tokens);
        parser.parse_program()
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, LangError> {
        let mut abstract_syntax_tree: Vec<Statement> = Vec::new();

        while !self.is_at_end() {
            if self.get_current_token() == Token::EndOfLine {
                self.advance();
                continue;
            } else if self.get_current_token() == Token::EndOfFile {
                break;
            }

            let statement: Statement = self.parse_statement()?;
            abstract_syntax_tree.push(statement);
        }

        Ok(abstract_syntax_tree)
    }

    fn parse_statement(&mut self) -> Result<Statement, LangError> {
        let token: Token = self.get_current_token();

        match token {
            Token::Identifier(identifier) => self.handle_variable_assignation(identifier),
            Token::Keyword(Keyword::Variable) => self.handle_variable_declaration(),
            Token::Keyword(Keyword::Print) => self.handle_print(),
            Token::Keyword(Keyword::If) => self.handle_condition(),
            Token::EndOfLine => Err(LangError::Parser(ParserError::InvalidSyntax(
                "Unexpected end of line".to_string(),
            ))),
            Token::EndOfFile => Err(LangError::Parser(ParserError::InvalidSyntax(
                "Unexpected end of file".to_string(),
            ))),
            _ => Err(LangError::from(ParserError::NotImplemented(
                "Token parsement not yet implemented".to_string(),
            ))),
        }
    }

    pub fn parse_block(&mut self) -> Result<Vec<Statement>, LangError> {
        self.advance_expecting(Token::Bracket(Side::Left))?;

        let mut statements: Vec<Statement> = Vec::new();

        while !self.is_at_end() && self.get_current_token() != Token::Bracket(Side::Right) {
            let statement: Statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.advance_expecting(Token::Bracket(Side::Right))?;

        Ok(statements)
    }

    pub fn parse_expression(&mut self) -> Result<Expression, LangError> {
        self.parse_term()
    }

    pub fn parse_term(&mut self) -> Result<Expression, LangError> {
        let mut expression: Expression = self.parse_factor()?;

        while let Token::Operator(operator) = self.get_current_token() {
            if operator.is_binary() {
                self.advance();
                let right: Expression = self.parse_factor()?;

                expression = Expression::Binary {
                    left: Box::new(expression),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<Expression, LangError> {
        let mut expression: Expression = self.parse_unary()?;

        while let Token::Operator(operator) = self.get_current_token() {
            if operator.is_multiplicative() {
                self.advance();
                let right: Expression = self.parse_unary()?;

                expression = Expression::Binary {
                    left: Box::new(expression),
                    operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Expression, LangError> {
        match self.get_current_token() {
            Token::Operator(Operator::Substraction) => {
                self.advance();

                let expression: Expression = self.parse_unary()?;

                Ok(Expression::Unary {
                    operator: Operator::Substraction,
                    expression: Box::new(expression),
                })
            }
            Token::Operator(Operator::Negation) => {
                self.advance();

                let expression: Expression = self.parse_unary()?;

                Ok(Expression::Unary {
                    operator: Operator::Negation,
                    expression: Box::new(expression),
                })
            }

            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, LangError> {
        let token: Token = self.advance();

        match token {
            Token::Literal(literal) => Ok(Expression::Literal(literal)),
            Token::Identifier(name) => Ok(Expression::Identifier(name)),
            _ => Err(ParserError::NotFound(format!("Unexpected token {token}")).into()),
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
            return Err(ParserError::InvalidSyntax(format!(
                "Expected {expected_token}, found {token}",
            ))
            .into());
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
