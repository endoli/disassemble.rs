// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use capstone_rust::capstone;
use capstone_rust::capstone_sys::{x86_insn, x86_insn_group};
use super::address::Address;
use super::instruction::Instruction;

fn is_group_match<F>(i: &capstone::Instr, predicate: F) -> bool
where
    F: Fn(&&u32) -> bool,
{
    match i.detail {
        None => false,
        Some(ref detail) => {
            match i.id {
                capstone::InstrIdArch::X86(_) => {
                    match detail.groups.iter().find(predicate) {
                        Some(_) => true,
                        None => false,
                    }
                }
                _ => false, // XXX Not supported yet
            }
        }
    }
}


impl Instruction for capstone::Instr {
    fn address(&self) -> Address {
        Address::new(self.address)
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }

    fn is_call(&self) -> bool {
        is_group_match(self, |&&x| x == x86_insn_group::X86_GRP_CALL.as_int())
    }

    fn is_local_conditional_jump(&self) -> bool {
        self.is_local_jump() &&
            match self.id {
                capstone::InstrIdArch::X86(x86_insn::X86_INS_JMP) => false,
                capstone::InstrIdArch::X86(x86_insn::X86_INS_LOOP) => false,
                capstone::InstrIdArch::X86(x86_insn::X86_INS_LOOPE) => false,
                capstone::InstrIdArch::X86(x86_insn::X86_INS_LOOPNE) => false,
                capstone::InstrIdArch::X86(x86_insn::X86_INS_XBEGIN) => false,
                capstone::InstrIdArch::X86(_) => true,
                _ => false, // XXX Not supported yet
            }
    }

    fn is_local_jump(&self) -> bool {
        is_group_match(self, |&&x| x == x86_insn_group::X86_GRP_JUMP.as_int()) &&
            match self.id {
                capstone::InstrIdArch::X86(x86_insn::X86_INS_LJMP) => false,
                capstone::InstrIdArch::X86(_) => true,
                _ => false, // XXX Not supported yet
            }
    }

    fn is_return(&self) -> bool {
        is_group_match(self, |&&x| x == x86_insn_group::X86_GRP_RET.as_int())
    }

    fn target_address(&self) -> Option<Address> {
        None
    }
}
