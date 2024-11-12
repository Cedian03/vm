//! Atom Assembler

mod lexer;
mod parser;
mod compiler;

use clap::Parser as Clap;
use logos::Logos;

use lexer::Token;
use parser::Parser;
use compiler::Compiler;

#[derive(Clap)]
struct Args {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let source = std::fs::read_to_string("./example.asm")?;

    let lexer = Token::lexer(&source);
    let tokens: Vec<_> = lexer.collect::<Result<_, _>>()?;

    dbg!(&tokens);

    let parser = Parser::new(&tokens);
    let statements: Vec<_> = parser.collect::<Result<_, _>>()?;

    dbg!(&statements);

    let mut compiler = Compiler::new(&statements);
    let _ = compiler.compile();

    Ok(())
}