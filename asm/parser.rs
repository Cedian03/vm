use std::str::FromStr;

use derive_more::{Display, Error};

use crate::lexer::Token;

#[derive(Clone, Copy, Debug, Default, Display, Error, PartialEq)]
pub enum ParsingError {
    #[default]
    Undefined
}

#[derive(Clone, Debug)]
pub enum Statement<'source> {
    Label(&'source str)
}

#[derive(Clone, Debug)]
pub struct Parser<'source> {
    tokens: &'source [Token<'source>],
    current: usize,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: &'source [Token]) -> Self {
        Self { tokens, current: 0 }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement<'a>, ParsingError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}