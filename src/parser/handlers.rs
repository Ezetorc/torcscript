use crate::{
    abstract_syntax_tree::{expression::Expression, literal::Literal, statement::Statement},
    errors::{lang_error::LangError, parser_error::ParserError},
    lexer::{keyword::Keyword, token::Token},
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

    pub fn handle_variable_assignation(
        &mut self,
        identifier: String,
    ) -> Result<Statement, LangError> {
        self.advance();

        self.advance_expecting(Token::Equal)?;

        let expression: Expression = self.parse_expression()?;

        Ok(Statement::VariableAssignation {
            identifier,
            expression,
        })
    }

    pub fn handle_commentary(&mut self) {
        self.advance();

        while !self.is_at_end() && self.get_current_token() != Token::EndOfLine {
            self.advance();
        }

        self.advance();
    }

    pub fn handle_variable_declaration(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let identifier_token: Token = self.advance();

        match identifier_token {
            Token::Identifier(identifier) => {
                let token: Token = self.advance();

                let expression: Expression = if token == Token::Equal {
                    self.parse_expression()?
                } else {
                    Expression::Literal(Literal::None)
                };

                Ok(Statement::VariableDeclaration {
                    identifier,
                    expression,
                })
            }
            _ => Err(ParserError::InvalidSyntax(format!(
                "Expected variable identifier, found {:?}",
                identifier_token
            ))
            .into()),
        }
    }
}
