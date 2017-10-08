// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use capstone_rust::capstone;
use capstone_rust::capstone_sys::{x86_insn, x86_insn_group};
use std::fmt;
use super::address::Address;
use super::instruction::Instruction;

/// A representation of an eBPF instruction.
#[derive(Debug)]
pub struct CapstoneInstruction {
    insn: capstone::Instr,
}

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
                _ => unimplemented!(),
            }
        }
    }
}


impl Instruction for CapstoneInstruction {
    fn address(&self) -> Address {
        Address::new(self.insn.address)
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn mnemonic(&self) -> &str {
        &*self.insn.mnemonic
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }

    fn is_call(&self) -> bool {
        is_group_match(&self.insn, |&&x| x == x86_insn_group::X86_GRP_CALL.as_int())
    }

    fn is_local_conditional_jump(&self) -> bool {
        self.is_local_jump() &&
            match self.insn.id {
                capstone::InstrIdArch::X86(x86_insn::X86_INS_JMP) |
                capstone::InstrIdArch::X86(x86_insn::X86_INS_LOOP) |
                capstone::InstrIdArch::X86(x86_insn::X86_INS_LOOPE) |
                capstone::InstrIdArch::X86(x86_insn::X86_INS_LOOPNE) |
                capstone::InstrIdArch::X86(x86_insn::X86_INS_XBEGIN) |
                capstone::InstrIdArch::X86(_) => true,
                _ => unimplemented!(),
            }
    }

    fn is_local_jump(&self) -> bool {
        is_group_match(&self.insn, |&&x| x == x86_insn_group::X86_GRP_JUMP.as_int()) &&
            match self.insn.id {
                capstone::InstrIdArch::X86(x86_insn::X86_INS_LJMP) => false,
                capstone::InstrIdArch::X86(_) => true,
                _ => unimplemented!(),
            }
    }

    fn is_return(&self) -> bool {
        is_group_match(&self.insn, |&&x| x == x86_insn_group::X86_GRP_RET.as_int())
    }

    fn target_address(&self) -> Option<Address> {
        None
    }
}

impl fmt::Display for CapstoneInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.insn.mnemonic, self.insn.op_str)
    }
}

#[cfg(test)]
mod tests {
    use capstone_rust::capstone as cs;
    use super::CapstoneInstruction;
    use super::super::{Address, Function, Symbol};

    #[test]
    fn test() {
        let code = &[0x55, 0x48, 0x8b, 0x05, 0xb8, 0x13, 0x00, 0x00];

        let dec = cs::Capstone::new(cs::cs_arch::CS_ARCH_X86, cs::cs_mode::CS_MODE_32).unwrap();
        let buf = dec.disasm(code, 0, 0).unwrap();
        let is = buf.iter()
            .map(|insn| CapstoneInstruction { insn })
            .collect::<Vec<_>>();
        let f = Function::new(Symbol::new(Address::new(100000), Some("test")), is);

        assert!(f.control_flow_graph.entry_block.is_some());
    }
}
