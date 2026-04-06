use crate::errors::lang_error::LangError;
use crate::errors::lexer_error::LexerError;

use crate::frontend::lexer::token::constructor::Constructor;
use crate::frontend::lexer::token::keyword::Keyword;
use crate::frontend::lexer::token::literal::Literal;
use crate::frontend::lexer::token::operator::Operator;
use crate::frontend::lexer::token::token::Token;
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
                lexer.handle_symbol();
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
            "Object" => Some(Token::Constructor(Constructor::Object)),
            "String" => Some(Token::Constructor(Constructor::String)),
            "False" => Some(Token::Literal(Literal::Boolean(false))),
            "True" => Some(Token::Literal(Literal::Boolean(true))),
            "List" => Some(Token::Constructor(Constructor::List)),
            "isnt" => Some(Token::Operator(Operator::Difference)),
            "not" => Some(Token::Operator(Operator::Negation)),
            "is" => Some(Token::Operator(Operator::Equality)),
            "action" => Some(Token::Keyword(Keyword::Function)),
            "print" => Some(Token::Keyword(Keyword::Print)),
            "print_named" => Some(Token::Keyword(Keyword::PrintNamed)),
            "state" => Some(Token::Keyword(Keyword::Variable)),
            "fact" => Some(Token::Keyword(Keyword::Constant)),
            "None" => Some(Token::Literal(Literal::None)),
            "else" => Some(Token::Keyword(Keyword::Else)),
            "and" => Some(Token::Operator(Operator::And)),
            "or" => Some(Token::Operator(Operator::Or)),
            "if" => Some(Token::Keyword(Keyword::If)),
            "in" => Some(Token::Keyword(Keyword::In)),
            "for" => Some(Token::Keyword(Keyword::For)),
            _ => None,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn get_current_char(&self) -> char {
        self.source[self.current]
    }

    pub fn get_next_char(&self) -> Option<char> {
        if self.current + 1 < self.source.len() {
            Some(self.source[self.current + 1])
        } else {
            None
        }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }
}
