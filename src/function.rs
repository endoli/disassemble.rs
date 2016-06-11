// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use basicblock::BasicBlock;
use instruction::Instruction;

#[allow(missing_docs)]
#[derive(Debug)]
pub struct Function<'f> {
    pub name: String,
    pub instructions: Vec<Box<Instruction>>,
    pub basic_blocks: Vec<BasicBlock<'f>>,
}

impl<'f> Function<'f> {
    /// Build the actual basic blocks for this function.
    ///
    /// This usually happens during construction of the `Function`.
    pub fn build_basic_blocks(&'f mut self) {
        // For now, let's just put all instructions into a single basic
        // block. In the future, we'll implement this correctly.
        let mut bb = BasicBlock::new(self.basic_blocks.len(), "anonymous");
        for inst in &self.instructions {
            bb.instructions.push(&inst);
        }
        self.basic_blocks.push(bb);
    }
}
