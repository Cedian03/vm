use derive_more::{Display, Error};
use logos::Logos;

#[derive(Clone, Copy, Debug, Default, Display, Error, PartialEq)]
pub enum LexingError {
    #[default]
    Undefined
}

#[derive(Clone, Copy, Debug, Logos)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token<'source> {
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[regex(r#"[a-z_][a-z0-9_]*"#, |lex| lex.slice())]
    Ident(&'source str),
    #[regex(r#"[0-9]+"#, |lex| lex.slice().parse::<u32>().unwrap())]
    Number(u32),
    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice())]
    String(&'source str),
}
