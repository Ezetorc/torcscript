use std::fmt;

use colored::Colorize;

use crate::frontend::token::{
    constructor::Constructor, keyword::Keyword, literal::Literal, operator::Operator, side::Side,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Operator(Operator),
    Literal(Literal),
    Keyword(Keyword),
    Bracket(Side),
    Parenthesis(Side),
    Constructor(Constructor),
    Commentary,
    EndOfLine,
    EndOfFile,
    Comma,
    Dot,
    Colon,
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(identifier) => write!(
                formatter,
                "{}{}{}",
                "Identifier(".blue(),
                identifier.italic(),
                ")".blue()
            ),

            Token::Operator(operator) => write!(
                formatter,
                "{}{}{}",
                "Operator(".blue(),
                operator.to_string().bright_yellow(),
                ")".blue()
            ),

            Token::Literal(literal) => write!(
                formatter,
                "{}{}{}",
                "Literal(".blue(),
                literal.to_string().italic(),
                ")".blue()
            ),

            Token::Keyword(keyword) => write!(
                formatter,
                "{}{}{}",
                "Keyword(".blue(),
                keyword.to_string().bright_yellow(),
                ")".blue()
            ),

            Token::Bracket(side) => write!(
                formatter,
                "{}{}{}",
                "Bracket(".cyan(),
                side.to_string().italic(),
                ")".cyan()
            ),

            Token::Parenthesis(side) => write!(
                formatter,
                "{}{}{}",
                "Parenthesis(".cyan(),
                side.to_string().italic(),
                ")".cyan()
            ),

            Token::Commentary => write!(formatter, "{}", "Commentary".bright_black()),

            Token::EndOfLine => write!(formatter, "{}", "EndOfLine".bright_black()),

            Token::EndOfFile => write!(formatter, "{}", "EndOfFile".black()),

            Token::Constructor(constructor) => write!(
                formatter,
                "{}{}{}",
                "Constructor(".blue(),
                constructor.to_string().yellow(),
                ")".blue()
            ),

            Token::Comma => write!(formatter, "{}", "Comma".blue()),

            Token::Dot => write!(formatter, "{}", "Dot".blue()),

            Token::Colon => write!(formatter, "{}", "Colon".blue()),
        }
    }
}
