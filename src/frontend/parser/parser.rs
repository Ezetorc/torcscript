use crate::{
    errors::{lang_error::LangError, parser_error::ParserError},
    frontend::lexer::token::{
        constructor::Constructor, keyword::Keyword, operator::Operator, side::Side, token::Token,
    },
    runtime::program::{expression::Expression, program::Program, statement::Statement},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Program, LangError> {
        let mut parser: Parser = Parser::new(tokens);

        parser.parse_tokens()
    }

    pub fn parse_tokens(&mut self) -> Result<Program, LangError> {
        let mut program: Program = Program::new();

        while !self.is_at_end() {
            let current_token: Token = self.get_current_token();

            match current_token {
                Token::EndOfFile => break,
                Token::EndOfLine => {
                    self.advance();
                    continue;
                }
                _ => {
                    let statement: Option<Statement> = self.parse_token()?;

                    if let Some(statement) = statement {
                        program.add(statement);
                    }
                }
            }
        }

        Ok(program)
    }

    fn parse_token(&mut self) -> Result<Option<Statement>, LangError> {
        let token: Token = self.get_current_token();

        match token {
            Token::Keyword(Keyword::Action) => Ok(Some(self.handle_action_declaration()?)),
            Token::Keyword(Keyword::State) => Ok(Some(self.handle_state_declaration()?)),
            Token::Keyword(Keyword::Print) => Ok(Some(self.handle_print()?)),
            Token::Keyword(Keyword::If) => Ok(Some(self.handle_condition()?)),
            Token::Commentary => {
                self.handle_commentary();
                Ok(None)
            }
            Token::EndOfLine => Ok(None),
            _ => {
                let expression: Expression = self.parse_expression()?;

                Ok(Some(Statement::Expression { expression }))
            }
        }
    }

    pub fn parse_block(&mut self) -> Result<Vec<Statement>, LangError> {
        self.advance_expecting(Token::Bracket(Side::Left))?;

        let mut statements: Vec<Statement> = Vec::new();

        while !self.is_at_end() && !self.current_is(Token::Bracket(Side::Right)) {
            let statement: Option<Statement> = self.parse_token()?;

            if let Some(statement) = statement {
                statements.push(statement);
            } else {
                self.advance();
            }
        }

        self.advance_expecting(Token::Bracket(Side::Right))?;

        Ok(statements)
    }

    pub fn parse_expression(&mut self) -> Result<Expression, LangError> {
        let left: Expression = self.parse_term()?;

        if self.current_is(Token::Operator(Operator::Equal)) {
            self.advance();

            let value: Expression = self.parse_expression()?;

            return Ok(Expression::Assignment {
                target: Box::new(left),
                expression: Box::new(value),
            });
        }

        Ok(left)
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

            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, LangError> {
        let mut expression: Expression = self.parse_primary()?;

        loop {
            match self.get_current_token() {
                Token::Dot => {
                    self.advance();

                    let name: String = match self.get_current_token() {
                        Token::Identifier(name) => name,
                        _ => {
                            return Err(ParserError::InvalidSyntax(
                                "Expected property name".into(),
                            )
                            .into());
                        }
                    };

                    self.advance();

                    expression = Expression::Member {
                        expression: Box::new(expression),
                        property: name,
                    };
                }

                Token::Parenthesis(Side::Left) => {
                    self.advance();

                    let arguments: Vec<Expression> = self.parse_arguments()?;

                    expression = Expression::Call {
                        callee: Box::new(expression),
                        arguments,
                    };
                }

                _ => break,
            }
        }

        Ok(expression)
    }

    fn parse_primary(&mut self) -> Result<Expression, LangError> {
        let token: Token = self.advance();

        match token {
            Token::Literal(literal) => Ok(Expression::Literal(literal)),
            Token::Identifier(name) => Ok(Expression::Identifier(name)),
            Token::Constructor(Constructor::Object) => Ok(self.handle_object()?),
            Token::Constructor(Constructor::List) => Ok(self.handle_list()?),
            _ => Err(ParserError::NotFound(format!("Unexpected token '{token}'")).into()),
        }
    }

    pub fn parse_identifier(&mut self, error_message: &str) -> Result<String, LangError> {
        match self.advance() {
            Token::Identifier(identifier) => Ok(identifier),
            _ => Err(ParserError::InvalidSyntax(error_message.to_string()).into()),
        }
    }

    pub fn parse_parameters_identifiers(&mut self) -> Result<Vec<String>, LangError> {
        let mut parameters: Vec<String> = Vec::new();

        if self.current_is(Token::Parenthesis(Side::Right)) {
            self.advance();

            return Ok(parameters);
        }

        while !self.is_at_end() && !self.current_is(Token::Parenthesis(Side::Right)) {
            if self.current_is(Token::Comma) {
                self.advance();
            }

            let name: String = self.parse_identifier("Expected parameter name")?;
            parameters.push(name);
        }

        self.advance_expecting(Token::Parenthesis(Side::Right))?;

        Ok(parameters)
    }

    pub fn parse_arguments(&mut self) -> Result<Vec<Expression>, LangError> {
        let mut arguments: Vec<Expression> = Vec::new();

        if self.current_is(Token::Parenthesis(Side::Right)) {
            self.advance();
            return Ok(arguments);
        }

        while !self.is_at_end() && !self.current_is(Token::Parenthesis(Side::Right)) {
            if self.current_is(Token::Comma) {
                self.advance();
            }

            let expression: Expression = self.parse_expression()?;
            arguments.push(expression);
        }

        self.advance_expecting(Token::Parenthesis(Side::Right))?;

        Ok(arguments)
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

    pub fn current_is(&self, token: Token) -> bool {
        self.get_current_token() == token
    }

    pub fn is_at_end(&self) -> bool {
        self.current_is(Token::EndOfFile)
    }
}
