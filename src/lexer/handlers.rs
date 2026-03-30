use crate::utilities::char_extension::CharExtension;
use crate::{
    abstract_syntax_tree::literal::Literal,
    lexer::{lexer::Lexer, token::Token},
};

impl Lexer {
    pub fn handle_string(&mut self) {
        self.advance();
        self.start = self.current;

        while !self.is_at_end() && !self.get_current_char().is_quote() {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();

        self.add_token(Token::Literal(Literal::String(value)));
        self.advance();
    }

    pub fn handle_alphabetic(&mut self) {
        self.start = self.current;

        while !self.is_at_end() && self.get_current_char().is_letter() {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let token: Option<Token> = Lexer::get_token_from(value.as_str());

        if let Some(token) = token {
            self.add_token(token);
        } else {
            self.add_token(Token::Identifier(value));
        }
    }

    pub fn handle_number(&mut self) {
        self.start = self.current;

        while !self.is_at_end() && self.get_current_char().is_number() {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let number: i64 = value.parse().unwrap();

        self.add_token(Token::Literal(Literal::Number(number)));
    }

    pub fn handle_symbol(&mut self) {
        self.start = self.current;

        while !self.is_at_end() && self.get_current_char().is_symbol() {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let token: Option<Token> = Lexer::get_token_from(value.as_str());

        if let Some(token) = token {
            self.add_token(token);
        }
    }
}
