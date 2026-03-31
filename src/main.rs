use crate::{
    abstract_syntax_tree::statement::Statement,
    errors::lang_error::LangError,
    interpreter::interpreter::Interpreter,
    lexer::{lexer::Lexer, token::Token},
    parser::parser::Parser,
    utilities::read_file::read_file,
};

mod abstract_syntax_tree;
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod utilities;

fn main() {
    if let Err(error) = run() {
        print!("{}", error);
    }
}

fn run() -> Result<(), LangError> {
    let source: String = read_file("src/examples/main.torc");

    let tokens: Vec<Token> = Lexer::tokenize(source)?;
    print!("Tokens: ");

    println!(
        "[{}]",
        tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let statements: Vec<Statement> = Parser::parse(tokens)?;
    print!("Statements: ");

    println!(
        "[{}]",
        statements
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    Interpreter::execute(statements)?;
    println!("Program finished");

    Ok(())
}
