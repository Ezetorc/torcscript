use crate::{
    frontend::lexer::{
        lexer::Lexer,
        token::{literal::Literal, operator::Operator, side::Side, token::Token},
    },
    utilities::char_extension::CharExtension,
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
        let current: char = self.get_current_char();
        let next: Option<char> = self.get_next_char();

        if let Some(token) = self.match_two_char_operator(current, next) {
            self.advance();
            self.advance();
            self.add_token(token);
            return;
        }

        if let Some(token) = self.match_one_char_symbol(current) {
            self.advance();
            self.add_token(token);
        }
    }

    fn match_two_char_operator(
        &self,
        current_char: char,
        next_char: Option<char>,
    ) -> Option<Token> {
        match (current_char, next_char) {
            ('=', Some('=')) => Some(Token::Operator(Operator::Equality)),
            ('!', Some('=')) => Some(Token::Operator(Operator::Difference)),
            ('>', Some('=')) => Some(Token::Operator(Operator::GreaterOrEqual)),
            ('<', Some('=')) => Some(Token::Operator(Operator::LessOrEqual)),
            _ => None,
        }
    }

    fn match_one_char_symbol(&self, current_char: char) -> Option<Token> {
        Some(match current_char {
            '*' => Token::Operator(Operator::Multiplication),
            '-' => Token::Operator(Operator::Substraction),
            '+' => Token::Operator(Operator::Addition),
            '/' => Token::Operator(Operator::Division),
            '>' => Token::Operator(Operator::Greater),
            '=' => Token::Operator(Operator::Equal),
            '<' => Token::Operator(Operator::Less),
            ')' => Token::Parenthesis(Side::Right),
            '(' => Token::Parenthesis(Side::Left),
            '}' => Token::Bracket(Side::Right),
            '{' => Token::Bracket(Side::Left),
            '#' => Token::Commentary,
            ';' => Token::EndOfLine,
            ',' => Token::Comma,
            ':' => Token::Colon,
            '.' => Token::Dot,
            _ => return None,
        })
    }
}
