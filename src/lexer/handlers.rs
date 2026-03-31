use crate::abstract_syntax_tree::operator::Operator;
use crate::lexer::side::Side;
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
        let current_char: char = self.get_current_char();
        let next_char: Option<char> = self.get_next_char();

        let token: Token = match (current_char, next_char) {
            ('=', Some('=')) => {
                self.advance();
                self.advance();
                Token::Operator(Operator::Equality)
            }
            ('!', Some('=')) => {
                self.advance();
                self.advance();
                Token::Operator(Operator::Difference)
            }
            ('>', Some('=')) => {
                self.advance();
                self.advance();
                Token::Operator(Operator::GreaterOrEqual)
            }
            ('<', Some('=')) => {
                self.advance();
                self.advance();
                Token::Operator(Operator::LessOrEqual)
            }

            _ => {
                self.advance();

                match current_char {
                    ')' => Token::Parenthesis(Side::Right),
                    '(' => Token::Parenthesis(Side::Left),
                    '}' => Token::Bracket(Side::Right),
                    '{' => Token::Bracket(Side::Left),
                    ';' => Token::EndOfLine,
                    '+' => Token::Operator(Operator::Addition),
                    '-' => Token::Operator(Operator::Substraction),
                    '/' => Token::Operator(Operator::Division),
                    '*' => Token::Operator(Operator::Multiplication),
                    '>' => Token::Operator(Operator::Greater),
                    '<' => Token::Operator(Operator::Less),
                    '=' => Token::Operator(Operator::Equal),
                    ',' => Token::Comma,
                    '#' => Token::Commentary,
                    _ => return,
                }
            }
        };

        self.add_token(token);
    }
}
