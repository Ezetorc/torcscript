use crate::{
    errors::lang_error::LangError,
    frontend::{lexer::lexer::Lexer, parser::parser::Parser, token::token::Token},
    runtime::{interpreter::interpreter::Interpreter, program::program::Program},
    utilities::{print_vector::print_vector, read_file::read_file},
};

mod errors;
mod frontend;
mod runtime;
mod utilities;

fn main() {
    if let Err(error) = run() {
        print!("{}", error);
    }
}

fn run() -> Result<(), LangError> {
    let source: String = read_file("src/examples/main.torc");

    let tokens: Vec<Token> = Lexer::tokenize(source)?;
    print_vector(&tokens, "Tokens");

    let program: Program = Parser::parse(tokens)?;
    print_vector(&program.statements, "Statements");

    Interpreter::execute(program)?;

    Ok(())
}
