use crate::{
    errors::lang_error::LangError, interpreter::interpreter::Interpreter, lexer::lexer::Lexer,
    parser::parser::Parser, utilities::read_file::read_file,
};

mod abstract_syntax_tree;
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod utilities;

fn main() {
    if let Err(error) = run() {
        println!("{}", error);
    }
}

fn run() -> Result<(), LangError> {
    let source = read_file("src/examples/main.torc");

    let tokens = Lexer::tokenize(source)?;
    println!("Tokens: {:?}", tokens);

    let statements = Parser::evaluate(tokens)?;
    println!("Statements: {:?}", statements);

    Interpreter::execute(statements)?;
    println!("Program finished");

    Ok(())
}
