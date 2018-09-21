// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate capstone;

use self::capstone::arch::x86::X86Insn;
use self::capstone::prelude::*;
use self::capstone::{Insn, InsnGroupType};
use super::address::Address;
use super::instruction::Instruction;
use std::fmt;

/// A representation of a Capstone instruction.
#[derive(Debug)]
pub struct CapstoneInstruction<'i> {
    insn: Insn<'i>,
    cs: &'i Capstone<'i>,
}

impl<'i> CapstoneInstruction<'i> {
    fn is_group_match(&self, group: u32) -> bool {
        if let Ok(detail) = self.cs.insn_detail(&self.insn) {
            detail.groups().any(|g| u32::from(g.0) == group)
        } else {
            false
        }
    }
}

impl<'i> Instruction for CapstoneInstruction<'i> {
    fn address(&self) -> Address {
        Address::new(self.insn.address())
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn mnemonic(&self) -> &str {
        &*self.insn.mnemonic().unwrap()
    }

    fn is_call(&self) -> bool {
        self.is_group_match(InsnGroupType::CS_GRP_CALL)
    }

    #[allow(unsafe_code)]
    fn is_local_conditional_jump(&self) -> bool {
        self.is_local_jump() && match unsafe { ::std::mem::transmute(self.insn.id().0) } {
            X86Insn::X86_INS_JMP
            | X86Insn::X86_INS_LOOP
            | X86Insn::X86_INS_LOOPE
            | X86Insn::X86_INS_LOOPNE
            | X86Insn::X86_INS_XBEGIN => true,
            _ => false,
        }
    }

    #[allow(unsafe_code)]
    fn is_local_jump(&self) -> bool {
        self.is_group_match(InsnGroupType::CS_GRP_JUMP) && match unsafe {
            ::std::mem::transmute(self.insn.id().0)
        } {
            X86Insn::X86_INS_LJMP => false,
            _ => true,
        }
    }

    fn is_return(&self) -> bool {
        self.is_group_match(InsnGroupType::CS_GRP_RET)
    }

    fn target_address(&self) -> Option<Address> {
        None
    }
}

impl<'i> fmt::Display for CapstoneInstruction<'i> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.insn.mnemonic().unwrap(),
            self.insn.op_str().unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Address, Function, Symbol};
    use super::capstone::prelude::*;
    use super::CapstoneInstruction;

    #[test]
    fn test() {
        let code = &[0x55, 0x48, 0x8b, 0x05, 0xb8, 0x13, 0x00, 0x00];

        let mut cs = Capstone::new()
            .x86()
            .mode(arch::x86::ArchMode::Mode32)
            .syntax(arch::x86::ArchSyntax::Att)
            .detail(true)
            .build()
            .unwrap();

        let buf = cs.disasm_all(code, 0).unwrap();
        let is = buf
            .iter()
            .map(|insn| CapstoneInstruction { insn, cs: &cs })
            .collect::<Vec<_>>();
        let f = Function::new(Symbol::new(Address::new(100000), Some("test")), is);

        assert!(f.control_flow_graph.entry_block.is_some());
    }
}
