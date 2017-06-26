// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use address::Address;
use basicblock::{BasicBlock, BasicBlockEdge, EdgeType};
use disassembler::Disassembler;
use instruction::Instruction;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::BTreeMap;

/// A [control flow graph].
///
/// [control flow graph]: https://en.wikipedia.org/wiki/Control_flow_graph
pub struct ControlFlowGraph<'f, I: 'f + Instruction> {
    /// The [`Graph`] that stores the actual ControlFlowGraph
    ///
    /// [`Graph`]: ../petgraph/graph/struct.Graph.html
    pub graph: Graph<BasicBlock<'f, I>, BasicBlockEdge>,
    /// The [`NodeIndex`] for the entry [`BasicBlock`] for this function.
    ///
    /// [`BasicBlock`]: struct.BasicBlock.html
    /// [`NodeIndex`]: ../petgraph/graph/struct.NodeIndex.html
    pub entry_block: Option<NodeIndex>,
    /// Map an [address] to the corresponding [basic block].
    ///
    /// [address]: struct.Address.html
    /// [basic block]: struct.BasicBlock.html
    pub block_finder: BTreeMap<Address, NodeIndex>,
}

impl<'f, I: Instruction> ControlFlowGraph<'f, I> {
    /// Build the ControlFlowGraph from the [`instructions`].
    ///
    /// This is conducted in a 2 step process:
    ///
    /// * First, each instruction is examined to identify block boundaries.
    /// * Then, we go through each instruction again, looking for the
    ///   previously identified block boundaries and build the edges.
    ///
    /// This two step process prevents us from having to construct and then
    /// subsequently split blocks as we find backward edges.
    ///
    /// [`instructions`]: trait.Instruction.html
    pub fn new(instructions: &'f [I], disassembler: &Disassembler) -> Self {
        let mut cfg = ControlFlowGraph {
            graph: Graph::new(),
            entry_block: None,
            block_finder: BTreeMap::new(),
        };
        if !instructions.is_empty() {
            cfg.identify_blocks(instructions, disassembler);
            cfg.build_edges(instructions, disassembler);
        }
        cfg
    }

    /// Identify basic blocks by their boundaries.
    ///
    /// Inspect each instruction to see if it is a 'leader' or the start of
    /// a new basic block. An instruction is a leader if:
    ///
    /// * It is the first instruction in the basic block.
    /// * It comes after a branch of any sort. We identify these as things
    ///   for which `Instruction::is_block_terminator` returns `true`.
    /// * It is the target of a jump (conditional or unconditional) within
    ///   the function.
    fn identify_blocks(&mut self, instructions: &[I], disassembler: &Disassembler) {
        let start_addr = instructions[0].address(disassembler);
        let end_addr = instructions.last().map(|i| i.address(disassembler)).unwrap();
        let mut next_is_leader: bool = true;
        for inst in instructions {
            if next_is_leader {
                let idx = self.graph.add_node(BasicBlock::new(inst.address(disassembler)));
                self.block_finder.insert(inst.address(disassembler), idx);
                next_is_leader = false;
            }
            if inst.is_block_terminator(disassembler) {
                if let Some(target_addr) = inst.target_address(disassembler) {
                    if !self.block_finder.contains_key(&target_addr) &&
                       target_addr >= start_addr && target_addr <= end_addr {
                        let idx = self.graph.add_node(BasicBlock::new(target_addr));
                        self.block_finder.insert(target_addr, idx);
                    }
                }
                // The next instruction, if any, will be the start of a new block.
                next_is_leader = true;
            }
        }
        self.entry_block = Some(*self.block_finder.get(&instructions[0].address(disassembler)).unwrap());
    }

    /// Build an edge between 2 basic blocks.
    fn build_edge(&mut self,
                  current_block_idx: NodeIndex,
                  next_block_idx: NodeIndex,
                  current_inst: &I,
                  disassembler: &Disassembler) {
        if current_inst.is_local_conditional_jump(disassembler) {
            // We have one edge for the jump target and one for the fallthrough.
            // We jump through some hoops here to keep the borrow checker happy.
            if let Some(target_addr) = current_inst.target_address(disassembler) {
                if let Some(target_block_idx) = self.block_finder.get(&target_addr) {
                    let edge = BasicBlockEdge { edge_type: EdgeType::ConditionalTaken };
                    self.graph.add_edge(current_block_idx, *target_block_idx, edge);
                }
                let edge = BasicBlockEdge { edge_type: EdgeType::ConditionalFallthrough };
                self.graph.add_edge(current_block_idx, next_block_idx, edge);
            }
        } else if current_inst.is_local_jump(disassembler) || current_inst.is_call(disassembler) {
            let edge = BasicBlockEdge { edge_type: EdgeType::Unconditional };
            self.graph.add_edge(current_block_idx, next_block_idx, edge);
        } else if current_inst.is_return(disassembler) {
            // Do we want to record this exit anywhere?
        }
    }

    /// Build edges after having identified the basic blocks.
    ///
    /// After the basic blocks have been identified, we can go back and
    /// connect the basic blocks with their appropriate edges.
    ///
    /// We do this by iterating through the instructions looking for
    /// boundaries between the basic blocks and then setting up the
    /// new edges.
    fn build_edges(&mut self, instructions: &'f [I], disassembler: &Disassembler) {
        // Here, we're going to walk through the instructions again,
        // looking at the current instruction, while also maintaining
        // a separate iterator giving us the next instruction (if there
        // is one.
        let current_inst_iter = instructions.iter();
        let mut next_inst_iter = current_inst_iter.clone();
        // Skip the first one so that we're actually working with the next one.
        next_inst_iter.next();
        let mut current_block_idx = self.entry_block.unwrap();
        for current_inst in current_inst_iter {
            // Add this instruction to the current block
            if let Some(current_block) = self.graph.node_weight_mut(current_block_idx) {
                current_block.instructions.push(current_inst);
            }
            if let Some(next_inst) = next_inst_iter.next() {
                // Does the next instruction begin a basic block?
                let next_block_idx =
                    *self.block_finder.get(&next_inst.address(disassembler)).unwrap_or(&current_block_idx);
                // If we're at a block boundary, create an edge between the
                // current and next blocks. The type of the edge is determined
                // by looking at the current instruction.
                if next_block_idx != current_block_idx {
                    self.build_edge(current_block_idx, next_block_idx, current_inst, disassembler);
                    current_block_idx = next_block_idx;
                }
            } else {
                // No next instruction, so we're at the end.
                // Do we want to record this exit anywhere?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::EdgeDirection;
    use super::ControlFlowGraph;
    use tests::*;

    #[test]
    fn construct() {
        let insts: Vec<TestInstruction> = vec![];
        let disasm = TestDisassembler::new();
        let cfg = ControlFlowGraph::new(&insts, &disasm);
        assert!(cfg.entry_block.is_none());
        assert_eq!(cfg.graph.node_count(), 0);
    }

    #[test]
    fn build_one_basic_block() {
        let insts = [TestInstruction::new(0, Opcode::Add),
                     TestInstruction::new(1, Opcode::Add),
                     TestInstruction::new(2, Opcode::Ret)];
        let disasm = TestDisassembler::new();
        let cfg = ControlFlowGraph::new(&insts, &disasm);
        assert!(cfg.entry_block.is_some());
        assert_eq!(cfg.graph.node_count(), 1);

        let inbound = cfg.graph.externals(EdgeDirection::Incoming);
        assert_eq!(inbound.count(), 1);

        let outbound = cfg.graph.externals(EdgeDirection::Outgoing);
        assert_eq!(outbound.count(), 1);
    }
}
