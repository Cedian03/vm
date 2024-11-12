use std::collections::HashMap;

use derive_more::{Display, Error};

use crate::parser::Statement;

#[derive(Clone, Copy, Debug, Default, Display, Error, PartialEq)]
pub enum CompilationError {
    #[default]
    Undefined
}

pub struct Compiler<'source> {
    statements: &'source [Statement<'source>],
    current: usize,

    code: Vec<u8>,
    data: Vec<u8>,

    labels: HashMap<&'source str, usize>,
}

impl<'a> Compiler<'a> {
    pub fn new(statements: &'a [Statement]) -> Self {
        Self {
            statements,
            current: 0,
            code: Vec::new(),
            data: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn compile(&mut self) -> Result<Vec<u8>, CompilationError> {
        todo!()
    }
}
