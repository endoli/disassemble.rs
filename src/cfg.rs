// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use basicblock::{BasicBlock, BasicBlockEdge};
use instruction::Instruction;
use petgraph::graph::{Graph, NodeIndex};

/// A [control flow graph].
///
/// [control flow graph]: https://en.wikipedia.org/wiki/Control_flow_graph
pub struct CFG<'f> {
    /// The [`Graph`] that stores the actual CFG
    ///
    /// [`Graph`]: ../petgraph/graph/struct.Graph.html
    pub graph: Graph<BasicBlock<'f>, BasicBlockEdge>,
    /// The [`NodeIndex`] for the entry [`BasicBlock`] for this function.
    ///
    /// [`BasicBlock`]: struct.BasicBlock.html
    /// [`NodeIndex`]: ../petgraph/graph/struct.NodeIndex.html
    pub entry_block: Option<NodeIndex>,
}

impl<'f> CFG<'f> {
    /// Construct a new CFG
    pub fn new() -> Self {
        CFG {
            graph: Graph::new(),
            entry_block: None,
        }
    }
    /// Build the CFG from the [`instructions`].
    ///
    /// [`instructions`]: trait.Instruction.html
    pub fn build(&mut self, instructions: &Vec<Box<Instruction>>) {
        if !instructions.is_empty() {
            let bb = BasicBlock::new(None, instructions[0].address());
            self.entry_block = Some(self.graph.add_node(bb));
        }
    }
}
