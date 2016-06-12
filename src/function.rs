// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use basicblock::{BasicBlock, BasicBlockEdge};
use instruction::Instruction;
use petgraph::graph::{Graph, NodeIndex};
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
    /// The [control flow graph] for this function. This is build from the
    /// `instructions` via `fn build_cfg`. It is made up of [basic blocks].
    ///
    /// [basic blocks]: struct.BasicBlock.html
    /// [control flow graph]: https://en.wikipedia.org/wiki/Control_flow_graph
    pub cfg: Option<Graph<BasicBlock<'f>, BasicBlockEdge>>,
    /// The `NodeIndex` for the entry [`BasicBlock`] for this function.
    ///
    /// [`BasicBlock`]: struct.BasicBlock.html
    pub entry_block: Option<NodeIndex>,
}

impl<'f> Function<'f> {
    /// Build the actual basic blocks for this function.
    ///
    /// This usually happens during construction of the `Function`.
    pub fn build_cfg(&'f mut self) {
        // For now, let's just put all instructions into a single basic
        // block. In the future, we'll implement this correctly.
        let mut cfg = Graph::new();
        let mut bb = BasicBlock::new(None, self.symbol.address);
        for inst in &self.instructions {
            bb.instructions.push(inst);
        }
        self.entry_block = Some(cfg.add_node(bb));
        self.cfg = Some(cfg);
    }
}
