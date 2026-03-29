use crate::abstract_syntax_tree::literal::Literal;
use crate::abstract_syntax_tree::operator::Operator;
use crate::errors::lang_error::LangError;
use crate::errors::lexer_error::LexerError;
use crate::lexer::keyword::Keyword;
use crate::lexer::side::Side;
use crate::lexer::token::Token;
use crate::utilities::char_extension::CharExtension;

pub struct Lexer {
    pub start: usize,
    pub current: usize,
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
}

impl Lexer {
    fn new(source: Vec<char>) -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
            start: 0,
            source,
        }
    }

    pub fn tokenize(source: String) -> Result<Vec<Token>, LangError> {
        let source: Vec<char> = source.chars().collect();
        let mut lexer: Lexer = Lexer::new(source);

        while !lexer.is_at_end() {
            let char: char = lexer.get_current_char();

            if char.is_quote() {
                lexer.handle_string();
            } else if char.is_letter() {
                lexer.handle_alphabetic();
            } else if char.is_number() {
                lexer.handle_number();
            } else if char.is_symbol() {
                lexer.handle_symbol(char);
            } else if char.is_skippable() {
                lexer.advance();
            } else {
                return Err(
                    LexerError::InvalidToken(format!("Token {char} not recognized")).into(),
                );
            }
        }

        lexer.tokens.push(Token::EndOfFile);

        Ok(lexer.tokens)
    }

    pub fn get_token_from(string: &str) -> Option<Token> {
        match string {
            "false" | "False" => Some(Token::Literal(Literal::Boolean(false))),
            "true" | "True" => Some(Token::Literal(Literal::Boolean(true))),
            "none" | "None" => Some(Token::Literal(Literal::None)),
            "*" => Some(Token::Operator(Operator::Multiplication)),
            "print" | "#" => Some(Token::Keyword(Keyword::Print)),
            "-" => Some(Token::Operator(Operator::Substraction)),
            "var" => Some(Token::Keyword(Keyword::Variable)),
            "+" => Some(Token::Operator(Operator::Addition)),
            "/" => Some(Token::Operator(Operator::Division)),
            "else" => Some(Token::Keyword(Keyword::Else)),
            "if" => Some(Token::Keyword(Keyword::If)),
            "is" => Some(Token::Operator(Operator::Equality)),
            "!" => Some(Token::Operator(Operator::Negation)),
            "}" => Some(Token::Bracket(Side::Right)),
            "{" => Some(Token::Bracket(Side::Left)),
            ";" => Some(Token::EndOfLine),
            "=" => Some(Token::Equal),
            _ => None,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn get_current_char(&self) -> char {
        self.source[self.current]
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }
}
