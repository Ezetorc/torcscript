use crate::{
    errors::lang_error::LangError,
    lexer::{lexer::Lexer, token::Token},
    utilities::read_file::read_file,
};

mod abstract_syntax_tree;
mod errors;
mod lexer;
mod parser;
mod utilities;

fn main() {
    let source: String = read_file("src/examples/main.torc");
    let tokens: Result<Vec<Token>, LangError> = Lexer::tokenize(source);

    println!("{:?}", tokens);
}
