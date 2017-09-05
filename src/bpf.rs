// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rbpf;

use self::rbpf::{disassembler, ebpf};
use super::address::Address;
use super::instruction::Instruction;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct BpfInstruction {
    idx: u64,
    insn: disassembler::HLInsn,
}

impl BpfInstruction {
    #[allow(missing_docs)]
    pub fn new(idx: u64, insn: disassembler::HLInsn) -> Self {
        BpfInstruction { idx, insn }
    }
}

impl Instruction for BpfInstruction {
    fn address(&self) -> Address {
        Address::new(self.idx)
    }

    fn comment(&self) -> Option<String> {
        None
    }

    fn mnemonic(&self) -> &str {
        &*self.insn.name
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }

    fn is_call(&self) -> bool {
        self.insn.opc == ebpf::CALL
    }

    fn is_local_conditional_jump(&self) -> bool {
        self.is_local_jump() && (self.insn.opc != ebpf::JA)
    }

    fn is_local_jump(&self) -> bool {
        (self.insn.opc & 0x07) == ebpf::BPF_JMP && (self.insn.opc != ebpf::CALL) &&
            (self.insn.opc != ebpf::TAIL_CALL) && (self.insn.opc != ebpf::EXIT)
    }

    fn is_return(&self) -> bool {
        self.insn.opc == ebpf::EXIT
    }

    fn target_address(&self) -> Option<Address> {
        if self.is_local_jump() {
            Some(Address::new(
                (self.idx as i64 + i64::from(self.insn.off)) as u64,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{rbpf, BpfInstruction};
    use super::super::{Address, Function, Symbol};

    #[test]
    fn test() {
        let prog = &[
            0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov r0, 0
            0x79, 0x12, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, // load ptr from r1[0x40] to r2
            0x07, 0x02, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, // add r2, 5
            0x79, 0x11, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, // load ptr from r1[0x50] to r1
            0x2d, 0x12, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, // if r2 > r1 skip 3 instructions
            0x71, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // load r2 (= *(mem + 5)) into r0
            0x67, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, // r0 >>= 56
            0xc7, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, // r0 <<= 56 (arsh)
            0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  // exit
        ];

        let v = rbpf::disassembler::to_insn_vec(prog);
        let is = v.into_iter()
            .enumerate()
            .map(|(idx, insn)| BpfInstruction::new(idx as u64, insn))
            .collect::<Vec<_>>();

        let f = Function::new(Symbol::new(Address::new(100000), Some("test")), is);

        assert!(f.control_flow_graph.entry_block.is_some());
        assert_eq!(f.control_flow_graph.graph.node_count(), 3);
    }
}
