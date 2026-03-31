use crate::{
    abstract_syntax_tree::{
        expression::Expression, literal::Literal, operator::Operator, statement::Statement,
    },
    errors::{lang_error::LangError, parser_error::ParserError},
    lexer::{keyword::Keyword, side::Side, token::Token},
    parser::parser::Parser,
};

impl Parser {
    pub fn handle_print(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let expression: Expression = self.parse_expression()?;

        if self.get_current_token() == Token::EndOfLine {
            self.advance();
        }

        Ok(Statement::Print { expression })
    }

    pub fn handle_action_declaration(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let identifier: String = self.parse_identifier("Expected action identifier")?;

        self.advance_expecting(Token::Parenthesis(Side::Left))?;

        let parameters: Vec<String> = self.parse_parameters_identifiers()?;
        let statements: Vec<Statement> = self.parse_block()?;

        Ok(Statement::ActionDeclaration {
            identifier,
            parameters,
            statements,
        })
    }

    pub fn handle_action_execution(&mut self, identifier: String) -> Result<Statement, LangError> {
        self.advance();

        self.advance_expecting(Token::Parenthesis(Side::Left))?;

        let parameters: Vec<Expression> = self.parse_arguments()?;

        Ok(Statement::ActionExecution {
            identifier,
            parameters,
        })
    }

    pub fn handle_condition(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let condition: Expression = self.parse_expression()?;
        let statements: Vec<Statement> = self.parse_block()?;

        if self.get_current_token() == Token::Keyword(Keyword::Else) {
            self.advance();

            if self.get_current_token() == Token::Keyword(Keyword::If) {
                let else_if_statement: Statement = self.handle_condition()?;

                return Ok(Statement::Conditional {
                    condition,
                    statements,
                    else_statements: Some(vec![else_if_statement]),
                });
            }

            let else_statements: Vec<Statement> = self.parse_block()?;

            return Ok(Statement::Conditional {
                condition,
                statements,
                else_statements: Some(else_statements),
            });
        }

        Ok(Statement::Conditional {
            condition,
            statements,
            else_statements: None,
        })
    }

    pub fn handle_state_assignation(&mut self, identifier: String) -> Result<Statement, LangError> {
        self.advance();

        self.advance_expecting(Token::Operator(Operator::Equal))?;

        let expression: Expression = self.parse_expression()?;

        Ok(Statement::StateAssignation {
            identifier,
            expression,
        })
    }

    pub fn handle_list(&mut self) -> Result<Expression, LangError> {
        self.advance_expecting(Token::Parenthesis(Side::Left))?;

        let mut elements = Vec::new();

        while self.get_current_token() != Token::Parenthesis(Side::Right) {
            elements.push(self.parse_expression()?);

            if self.get_current_token() == Token::Comma {
                self.advance();
            }
        }

        self.advance_expecting(Token::Parenthesis(Side::Right))?;

        Ok(Expression::List(elements))
    }

    pub fn handle_commentary(&mut self) {
        self.advance();

        while !self.is_at_end() && self.get_current_token() != Token::EndOfLine {
            self.advance();
        }

        self.advance();
    }

    pub fn handle_state_declaration(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let identifier_token: Token = self.advance();

        match identifier_token {
            Token::Identifier(identifier) => {
                let token: Token = self.advance();

                let expression: Expression = if token == Token::Operator(Operator::Equal) {
                    self.parse_expression()?
                } else {
                    Expression::Literal(Literal::None)
                };

                Ok(Statement::StateDeclaration {
                    identifier,
                    expression,
                })
            }
            _ => Err(ParserError::InvalidSyntax(format!(
                "Expected state identifier, found {:?}",
                identifier_token
            ))
            .into()),
        }
    }
}
