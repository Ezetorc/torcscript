use crate::{
    abstract_syntax_tree::{expression::Expression, statement::Statement},
    errors::lang_error::LangError,
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
}
