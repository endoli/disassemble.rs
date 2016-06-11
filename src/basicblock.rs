// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use instruction::Instruction;

/// A reference to a [`BasicBlock`].
///
/// [`BasicBlock`]: struct.BasicBlock.html
#[derive(Clone,Copy,Debug)]
pub struct BasicBlockRef {
    /// The ID for the referenced `BasicBlock`.
    pub id: usize,
}

/// A [basic block] is a sequence of instructions with no inward-bound
/// branches except to the entry point and no outward-bound branches
/// except at the exit.
///
/// [basic block]: https://en.wikipedia.org/wiki/Basic_block
#[derive(Debug)]
pub struct BasicBlock<'f> {
    /// The ID # for this basic block. This is artificial information and
    /// not something from the disassembly.
    pub id: usize,
    /// The name of the basic block. Not all blocks have meaningful names.
    pub name: String,
    /// The instructions within this basic block.
    pub instructions: Vec<&'f Box<Instruction>>,
    /// The basic blocks that point to this one.
    pub in_edges: Vec<BasicBlockRef>,
    /// The basic blocks which can be exited to from this one.
    pub out_edges: Vec<BasicBlockRef>,
}

impl<'f> BasicBlock<'f> {
    /// Construct a new `BasicBlock`.
    pub fn new(id: usize, name: &str) -> Self {
        BasicBlock {
            id: id,
            name: name.to_owned(),
            instructions: vec![],
            in_edges: vec![],
            out_edges: vec![],
        }
    }

    /// Add an edge that points to this basic block.
    pub fn add_in_edge(&mut self, bb: &BasicBlock<'f>) {
        self.in_edges.push(BasicBlockRef { id: bb.id });
    }

    /// Add an edge that points from this basic block to another.
    pub fn add_out_edge(&mut self, bb: &BasicBlock<'f>) {
        self.out_edges.push(BasicBlockRef { id: bb.id });
    }
}
