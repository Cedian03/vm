use derive_more::TryFrom;

macro_rules! define_operation_enums {
    (
        $(#[$p_meta:meta])*
        $p_vis:vis enum $p_name:ident {
            $($variant:ident { $($field_name:ident : $field_type:ty),* }),* $(,)?
        }

        $(#[$s_meta:meta])*
        $s_vis:vis enum $s_name:ident;
    ) => {
        $(#[$p_meta])*
        $p_vis enum $p_name {
            $($variant { $($field_name : $field_type),* },)*
        }

        $(#[$s_meta])*
        $s_vis enum $s_name {
            $($variant,)*
        }
    };
}

define_operation_enums!(
    #[derive(Clone, Debug)]
    pub enum Operation {
        Halt {},

        AddImm { src: Register, val: u32, dst: Register },

        Add { src: Register, dst: Register }, 
        Sub { src: Register, dst: Register }, 
        And { src: Register, dst: Register }, 
        Or  { src: Register, dst: Register }, 
        Xor { src: Register, dst: Register }, 

        Jump { addy: u32, dst: Register },

        BranchEQ { lhs: Register, rhs: Register, addy: u32 },
        BranchNE { lhs: Register, rhs: Register, addy: u32 },
        BranchLT { lhs: Register, rhs: Register, addy: u32 },
        BranchGE { lhs: Register, rhs: Register, addy: u32 },

        Load  { ady: u32, dst: Register },
        Store { src: Register, ady: u32 },
    }

    #[derive(Clone, Copy, Debug, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(u8)]
    pub enum OperationCode;
);

#[derive(Clone, Copy, Debug, PartialEq, TryFrom)]
#[try_from(repr)]
#[repr(u8)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}