use std::collections::HashMap;

use crate::{
    errors::{lang_error::LangError, parser_error::ParserError},
    frontend::{
        parser::parser::Parser,
        token::{keyword::Keyword, operator::Operator, side::Side, token::Token},
    },
    runtime::program::{expression::Expression, statement::Statement},
};

impl Parser {
    pub fn handle_print(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let expression: Expression = self.parse_expression()?;

        if self.current_is(Token::EndOfLine) {
            self.advance();
        }

        Ok(Statement::Print { expression })
    }

    pub fn handle_print_named(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let expression: Expression = self.parse_expression()?;

        if self.current_is(Token::EndOfLine) {
            self.advance();
        }

        match &expression {
            Expression::Identifier(identifier) => Ok(Statement::PrintNamed {
                name: identifier.clone(),
                expression,
            }),
            _ => Err(ParserError::InvalidSyntax(
                "'print_named' can be only used with identifiers".to_string(),
            )
            .into()),
        }
    }

    pub fn handle_function_declaration(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let identifier: String = self.parse_identifier("Expected action identifier")?;
        let parameters: Vec<String> = self.parse_parameters_identifiers()?;
        let statements: Vec<Statement> = self.parse_block()?;

        Ok(Statement::FunctionDeclaration {
            identifier,
            parameters,
            statements,
        })
    }

    pub fn handle_condition(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let condition: Expression = self.parse_expression()?;
        let statements: Vec<Statement> = self.parse_block()?;

        if !self.current_is(Token::Keyword(Keyword::Else)) {
            return Ok(Statement::Conditional {
                condition,
                statements,
                else_statements: None,
            });
        }

        self.advance();

        if self.current_is(Token::Keyword(Keyword::If)) {
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

    pub fn handle_for_loop(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let parameters: Vec<String> = self.parse_parameters_identifiers()?;

        self.advance_expecting(Token::Keyword(Keyword::In))?;

        let iterator: Expression = self.parse_expression()?;
        let statements: Vec<Statement> = self.parse_block()?;

        Ok(Statement::ForLoop {
            iterator,
            parameters,
            statements,
        })
    }

    pub fn handle_list(&mut self) -> Result<Expression, LangError> {
        self.advance_expecting(Token::Parenthesis(Side::Left))?;

        let mut elements: Vec<Expression> = Vec::new();

        while !self.current_is(Token::Parenthesis(Side::Right)) {
            elements.push(self.parse_expression()?);

            if self.current_is(Token::Comma) {
                self.advance();
            }
        }

        self.advance_expecting(Token::Parenthesis(Side::Right))?;

        Ok(Expression::List(elements))
    }

    pub fn handle_string(&mut self) -> Result<Expression, LangError> {
        self.advance_expecting(Token::Parenthesis(Side::Left))?;

        let expression: Expression = self.parse_expression()?;

        self.advance_expecting(Token::Parenthesis(Side::Right))?;

        Ok(Expression::String(Box::new(expression)))
    }

    pub fn handle_object(&mut self) -> Result<Expression, LangError> {
        self.advance_expecting(Token::Parenthesis(Side::Left))?;

        let mut fields: HashMap<String, Expression> = HashMap::new();

        while !self.is_at_end() && !self.current_is(Token::Parenthesis(Side::Right)) {
            let token: Token = self.advance();

            let identifier: String = match token {
                Token::Identifier(identifier) => identifier,
                _ => {
                    return Err(ParserError::InvalidSyntax(format!(
                        "Expected field name, found '{token}'"
                    ))
                    .into());
                }
            };

            self.advance_expecting(Token::Colon)?;

            let expression: Expression = self.parse_expression()?;
            fields.insert(identifier, expression);

            if self.current_is(Token::Comma) {
                self.advance();
            }
        }

        self.advance_expecting(Token::Parenthesis(Side::Right))?;

        Ok(Expression::Object(fields))
    }

    pub fn handle_commentary(&mut self) {
        self.advance();

        while !self.is_at_end() && !self.current_is(Token::EndOfLine) {
            self.advance();
        }

        self.advance();
    }

    pub fn handle_variable_declaration(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let identifier: String = self.parse_identifier("Expected state name")?;

        self.advance_expecting(Token::Operator(Operator::Equal))?;

        let expression: Expression = self.parse_expression()?;

        Ok(Statement::VariableDeclaration {
            identifier,
            expression,
        })
    }

    pub fn handle_constant_declaration(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let identifier: String = self.parse_identifier("Expected fact name")?;

        self.advance_expecting(Token::Operator(Operator::Equal))?;

        let expression: Expression = self.parse_expression()?;

        Ok(Statement::ConstantDeclaration {
            identifier,
            expression,
        })
    }
}
