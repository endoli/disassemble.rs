// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use instruction::Instruction;

/// When is this edge taken? Conditionally or unconditionally?
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum EdgeType {
    /// This edge is taken when a conditional is true.
    ConditionalTrue,
    /// This edge is taken when a conditional is false.
    ConditionalFalse,
    /// This is edge is always taken.
    Unconditional,
}

/// Which direction is this edge going? Inwards or outwards from this `BasicBlock`?
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum EdgeDirection {
    /// This is an inbound edge with this [`BasicBlock`] as the target.
    ///
    /// [`BasicBlock`]: struct.BasicBlock.html
    In,
    /// This is an outbound edge with this [`BasicBlock`] as the source.
    ///
    /// [`BasicBlock`]: struct.BasicBlock.html
    Out,
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
    pub name: Option<String>,
    /// The instructions within this basic block.
    pub instructions: Vec<&'f Box<Instruction>>,
    /// The basic blocks that point to this one.
    pub in_edges: Vec<BasicBlockEdge<'f>>,
    /// The basic blocks which can be exited to from this one.
    pub out_edges: Vec<BasicBlockEdge<'f>>,
}

impl<'f> BasicBlock<'f> {
    /// Construct a new `BasicBlock`.
    pub fn new(id: usize, name: Option<&str>) -> Self {
        BasicBlock {
            id: id,
            name: name.map(|s| s.to_owned()),
            instructions: vec![],
            in_edges: vec![],
            out_edges: vec![],
        }
    }

    /// Add an edge that points to this basic block.
    pub fn add_in_edge(&'f mut self, bb: &'f BasicBlock<'f>, edge_type: EdgeType) {
        self.in_edges.push(BasicBlockEdge {
            edge_type: edge_type,
            direction: EdgeDirection::In,
            other_bb: bb,
        });
    }

    /// Add an edge that points from this basic block to another.
    pub fn add_out_edge(&'f mut self, bb: &'f BasicBlock<'f>, edge_type: EdgeType) {
        self.out_edges.push(BasicBlockEdge {
            edge_type: edge_type,
            direction: EdgeDirection::Out,
            other_bb: bb,
        });
    }
}

/// Information about an edge between 2 [basic blocks].
///
/// This represents a branch, jump or other form of control flow
/// transfer within the control flow graph.
///
/// [basic blocks]: struct.BasicBlock.html
#[derive(Debug)]
pub struct BasicBlockEdge<'f> {
    /// Is this edge taken [conditionally or unconditionally]?
    ///
    /// [conditionally or unconditionally]: enum.EdgeType.html
    pub edge_type: EdgeType,
    /// Is this an [inbound or outbound] edge?
    ///
    /// [inbound or outbound]: enum.EdgeDirection.html
    pub direction: EdgeDirection,
    /// What is the other [`BasicBlock`] involved in this edge?
    ///
    /// If the `direction` is [`EdgeDirection::In`], then this `other_bb`
    /// will be the source bb and this bb will be the destination.
    ///
    /// If the `direction` is [`EdgeDirection::Out`], then this bb will
    /// be the source and `other_bb` will be the destination.
    ///
    /// [`BasicBlock`]: struct.BasicBlock.html
    /// [`EdgeDirection::In`]: enum.EdgeDirection.html
    /// [`EdgeDirection::Out`]: enum.EdgeDirection.html
    pub other_bb: &'f BasicBlock<'f>,
}
