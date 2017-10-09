// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate burst;

use std::fmt;
use super::address::Address;
use super::instruction::Instruction;

/// A representation of a Burst instruction.
#[derive(Debug)]
pub struct BurstInstruction {
    insn: burst::x86::Instruction,
}

impl Instruction for BurstInstruction {
    fn address(&self) -> Address {
        unimplemented!();
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn mnemonic(&self) -> &str {
        self.insn.operation.mnemonic()
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }

    fn is_call(&self) -> bool {
        use self::burst::x86::InstructionOperation::*;

        match self.insn.operation {
            CALL | CALLF => true,
            _ => false,
        }
    }

    fn is_local_conditional_jump(&self) -> bool {
        use self::burst::x86::InstructionOperation::*;

        match self.insn.operation {
            JCXZ | JECXZ | JO | JNO | JB | JAE | JE | JNE | JBE | JA | JS | JNS | JPE | JPO |
            JL | JGE | JLE | JG | LOOP | LOOPE | LOOPNE => true,
            _ => false,
        }
    }

    fn is_local_jump(&self) -> bool {
        use self::burst::x86::InstructionOperation::*;

        self.is_local_conditional_jump() ||
            match self.insn.operation {
                JMPF | JMP => true,
                _ => false,
            }
    }

    fn is_return(&self) -> bool {
        use self::burst::x86::InstructionOperation::*;

        match self.insn.operation {
            RETF | RETN => true,
            _ => false,
        }
    }

    fn target_address(&self) -> Option<Address> {
        let first_operand = &self.insn.operands[0];
        if first_operand.operand == burst::x86::OperandType::IMM {
            Some(Address::new(first_operand.immediate as u64))
        } else {
            None
        }
    }
}

impl fmt::Display for BurstInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data: [u8; 0] = [];
        let mut out = String::new();
        try!(burst::x86::format_instruction_string(
            &mut out,
            "%i %o",
            &data,
            0,
            &self.insn,
        ));
        out.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::burst;
    use super::BurstInstruction;
    use super::super::{Address, Function, Symbol};

    #[test]
    fn test() {
        let code = &[0x55, 0x48, 0x8b, 0x05, 0xb8, 0x13, 0x00, 0x00];

        let buf = burst::x86::disassemble_64(code, 0, 0);
        let is = buf.iter()
            .map(|insn| BurstInstruction { insn })
            .collect::<Vec<_>>();
        let f = Function::new(Symbol::new(Address::new(100000), Some("test")), is);

        assert!(f.control_flow_graph.entry_block.is_some());
    }
}
