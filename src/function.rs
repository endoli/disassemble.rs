// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use basicblock::BasicBlock;
use instruction::Instruction;
use symbol::Symbol;

/// A function within a program.
#[derive(Debug)]
pub struct Function<'f> {
    /// The [symbol] for this function. This provides the name and [`Address`].
    ///
    /// [`Address`]: struct.Symbol.html
    /// [symbol]: struct.Symbol.html
    pub symbol: Symbol,
    /// The [instructions] that comprise this function.
    ///
    /// [instructions]: trait.Instruction.html
    pub instructions: Vec<Box<Instruction>>,
    /// The [basic blocks] that comprise this function. These are algorithmically
    /// determined from the `instructions` via `fn build_basic_blocks`.
    ///
    /// The `basic_blocks` of a `Function` make up a [control flow graph].
    ///
    /// [basic blocks]: struct.BasicBlock.html
    /// [control flow graph]: https://en.wikipedia.org/wiki/Control_flow_graph
    pub basic_blocks: Vec<BasicBlock<'f>>,
    /// The entry [`BasicBlock`] for this function.
    ///
    /// [`BasicBlock`]: struct.BasicBlock.html
    pub entry_block: Option<&'f BasicBlock<'f>>,
}

impl<'f> Function<'f> {
    /// Build the actual basic blocks for this function.
    ///
    /// This usually happens during construction of the `Function`.
    pub fn build_basic_blocks(&'f mut self) {
        // For now, let's just put all instructions into a single basic
        // block. In the future, we'll implement this correctly.
        let mut bb = BasicBlock::new(self.basic_blocks.len(), None);
        for inst in &self.instructions {
            bb.instructions.push(inst);
        }
        self.basic_blocks.push(bb);
        self.entry_block = Some(&self.basic_blocks[0]);
    }
}
