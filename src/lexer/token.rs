use std::fmt::{Display, Result};

use colored::Colorize;

use crate::{
    abstract_syntax_tree::{literal::Literal, operator::Operator},
    lexer::{keyword::Keyword, side::Side},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Operator(Operator),
    Literal(Literal),
    Keyword(Keyword),
    Bracket(Side),
    Parenthesis(Side),
    Commentary,
    EndOfLine,
    EndOfFile,
    List,
    Object,
    Comma,
    Colon,
}

impl Display for Token {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result {
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

            Token::EndOfFile => write!(formatter, "{}", "EndOfFile".bright_red()),

            Token::List => write!(formatter, "{}", "List".blue()),

            Token::Object => write!(formatter, "{}", "Object".blue()),

            Token::Comma => write!(formatter, "{}", "Comma".blue()),

            Token::Colon => write!(formatter, "{}", "Colon".blue()),
        }
    }
}
