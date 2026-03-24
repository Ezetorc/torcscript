use crate::{
    errors::lang_error::LangError,
    frontend::{lexer::lexer::Lexer, token::token::Token},
    utilities::read_file::read_file,
};

mod errors;
mod frontend;
mod utilities;

fn main() {
    let source: String = read_file("src/examples/main.torc");
    let tokens: Result<Vec<Token>, LangError> = Lexer::tokenize(source);

    println!("{:?}", tokens);
}
