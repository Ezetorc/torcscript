use crate::{
    abstract_syntax_tree::{expression::Expression, statement::Statement},
    errors::{lang_error::LangError, parser_error::ParserError},
    lexer::token::Token,
    parser::parser::Parser,
};

impl Parser {
    pub fn handle_print(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let expression: Expression = self.parse_expression();

        if self.get_current_token() == Token::EndOfLine {
            self.advance();
        }

        Ok(Statement::Print { expression })
    }

    pub fn handle_variable_declaration(&mut self) -> Result<Statement, LangError> {
        self.advance();

        let identifier_token: Token = self.advance();

        match identifier_token {
            Token::Identifier(identifier) => {
                self.advance_expecting(Token::Equal)?;

                let expression: Expression = self.parse_expression();

                Ok(Statement::VariableDeclaration { identifier, expression })
            }
            _ => {
                return Err(LangError::Parser(ParserError::InvalidSyntax(
                    "Expected variable identifier".to_string(),
                )));
            }
        }
    }
}
