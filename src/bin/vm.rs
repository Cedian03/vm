//! Atom Virtual Machine

use std::ops;

use derive_more::TryFromReprError;

use vm::{Register, OperationCode, Operation};

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug)]
enum Error {
    UnexpectedEnd,
    Undefined,
}

impl From<TryFromReprError<u8>> for Error {
    fn from(_: TryFromReprError<u8>) -> Self {
        Error::Undefined
    }
}

const VM_MEMORY: usize = 1 << 20; // TODO: Dynamic?

#[derive(Debug)]
struct Machine {
    registers: Registers,
    memory: [u8; VM_MEMORY],
    ip: usize,
}

impl Machine {
    fn new(code: &[u8]) -> Self {
        let mut memory = [0; VM_MEMORY];    

        memory[..code.len()].copy_from_slice(code);

        Self {
            registers: Default::default(),
            memory, 
            ip: 0,
        }
    }

    fn interpret(&mut self) -> Result<()> {
        loop {
            match self.next()? {
                Operation::Halt {} => return Ok(()),

                Operation::AddImm { src, val, dst } => self.registers[dst] = self.registers[src] + val,

                Operation::Add { src, dst } => self.binary(u32::wrapping_add,   src, dst),
                Operation::Sub { src, dst } => self.binary(u32::wrapping_sub,   src, dst),
                Operation::And { src, dst } => self.binary(ops::BitAnd::bitand, src, dst),
                Operation::Or  { src, dst } => self.binary(ops::BitOr::bitor,   src, dst),
                Operation::Xor { src, dst } => self.binary(ops::BitXor::bitxor, src, dst),

                Operation::Jump { addy, dst } => { self.registers[dst] = self.ip as u32; self.ip = addy as usize; }

                Operation::BranchEQ { lhs, rhs, addy } => self.branch(PartialEq::eq,  lhs, rhs, addy),
                Operation::BranchNE { lhs, rhs, addy } => self.branch(PartialEq::ne,  lhs, rhs, addy),
                Operation::BranchLT { lhs, rhs, addy } => self.branch(PartialOrd::lt, lhs, rhs, addy),
                Operation::BranchGE { lhs, rhs, addy } => self.branch(PartialOrd::ge, lhs, rhs, addy),
                
                x => unimplemented!("{:?}", x),
            }                
        }
    }

    fn binary<F: Fn(u32, u32) -> u32>(&mut self, f: F, src: Register, dst: Register) {
        self.registers[dst] = f(self.registers[dst], self.registers[src])
    }

    fn branch<F: Fn(&u32, &u32) -> bool>(&mut self, f: F, lhs: Register, rhs: Register, ady: u32) {
        if f(&self.registers[lhs], &self.registers[rhs]) { self.ip = ady as usize }
    }

    fn next(&mut self) -> Result<Operation> {
        Ok(match self.next_byte()?.try_into()? {
            OperationCode::Halt => Operation::Halt {},

            OperationCode::AddImm => Operation::AddImm { src: self.next_byte()?.try_into()?, val: self.next_double()?, dst: self.next_byte()?.try_into()? },

            OperationCode::Add => Operation::Add { src: self.next_byte()?.try_into()?, dst: self.next_byte()?.try_into()? },
            OperationCode::Sub => Operation::Sub { src: self.next_byte()?.try_into()?, dst: self.next_byte()?.try_into()? },
            OperationCode::Xor => Operation::Xor { src: self.next_byte()?.try_into()?, dst: self.next_byte()?.try_into()? },
            OperationCode::Or  => Operation::Or  { src: self.next_byte()?.try_into()?, dst: self.next_byte()?.try_into()? },
            OperationCode::And => Operation::And { src: self.next_byte()?.try_into()?, dst: self.next_byte()?.try_into()? },

            OperationCode::Jump => Operation::Jump { addy: self.next_double()?, dst: self.next_byte()?.try_into()? },

            OperationCode::BranchEQ => Operation::BranchEQ { lhs: self.next_byte()?.try_into()?, rhs: self.next_byte()?.try_into()?, addy: self.next_double()? },
            OperationCode::BranchNE => Operation::BranchNE { lhs: self.next_byte()?.try_into()?, rhs: self.next_byte()?.try_into()?, addy: self.next_double()? },
            OperationCode::BranchLT => Operation::BranchLT { lhs: self.next_byte()?.try_into()?, rhs: self.next_byte()?.try_into()?, addy: self.next_double()? },
            OperationCode::BranchGE => Operation::BranchGE { lhs: self.next_byte()?.try_into()?, rhs: self.next_byte()?.try_into()?, addy: self.next_double()? },

            x => unimplemented!("{:?}", x),
        })
    }

    fn next_byte(&mut self) -> Result<u8> {
        if self.ip < self.memory.len() {
            let byte = self.memory[self.ip];
            self.ip += 1;
            Ok(byte)    
        } else {
            Err(Error::UnexpectedEnd)
        }
    }

    fn next_word(&mut self) -> Result<u16> {
        let a = self.next_byte()?;
        let b = self.next_byte()?;

        Ok(((a as u16) << 8) | (b as u16))
    }

    fn next_double(&mut self) -> Result<u32> {
        let a = self.next_word()?;
        let b = self.next_word()?;

        Ok(((a as u32) << 16) | (b as u32))
    }
}

#[derive(Debug, Default)]
struct Registers {
    inner: [u32; 16]
}

impl ops::Index<Register> for Registers {
    type Output = u32;

    fn index(&self, register: Register) -> &Self::Output {
        &self.inner[register as usize]
    }
}

impl ops::IndexMut<Register> for Registers {
    fn index_mut(&mut self, register: Register) -> &mut Self::Output {
        &mut self.inner[register as usize]

    }
}

fn main() -> Result<()> {
    let bytecode = &[
        OperationCode::AddImm as u8, Register::R0 as u8, 0x00, 0x00, 0x00, 0x01, Register::R1 as u8,
        OperationCode::AddImm as u8, Register::R0 as u8, 0x00, 0x00, 0x00, 0x00, Register::R2 as u8,
        OperationCode::AddImm as u8, Register::R0 as u8, 0x00, 0x00, 0x00, 0x0A, Register::R4 as u8,
        OperationCode::AddImm as u8, Register::R0 as u8, 0x00, 0x00, 0x00, 0x01, Register::R5 as u8,

        OperationCode::AddImm as u8, Register::R1 as u8, 0x00, 0x00, 0x00, 0x00, Register::R3 as u8,
        OperationCode::Add as u8, Register::R2 as u8, Register::R1 as u8, 
        OperationCode::AddImm as u8, Register::R3 as u8, 0x00, 0x00, 0x00, 0x00, Register::R2 as u8,
        OperationCode::Sub as u8, Register::R5 as u8, Register::R4 as u8,
        OperationCode::BranchGE as u8, Register::R4 as u8, Register::R5 as u8, 0x00, 0x00, 0x00, 0x1C,

        OperationCode::Halt as u8,
    ];


    let mut machine = Machine::new(bytecode);
    machine.interpret()?;
    dbg!(machine.registers);

    Ok(())
}
