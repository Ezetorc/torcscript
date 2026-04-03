use colored::Colorize;

use crate::{
    errors::lang_error::LangError,
    frontend::{
        abstract_syntax_tree::statement::Statement,
        lexer::{lexer::Lexer, token::token::Token},
        parser::parser::Parser,
    },
    runtime::interpreter::interpreter::Interpreter,
    utilities::read_file::read_file,
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
    print!("{}", "Tokens: ".green().bold());

    println!(
        "[{}]",
        tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let statements: Vec<Statement> = Parser::parse(tokens)?;
    print!("{}", "Statements: ".green().bold());

    println!(
        "[{}]",
        statements
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    Interpreter::execute(statements)?;

    Ok(())
}
