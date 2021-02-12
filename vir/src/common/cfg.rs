use std::borrow::Cow;
use std::collections::VecDeque;

use index_vec::{Idx, IndexVec};

pub trait BasicBlockWithSuccessors {
    type BasicBlockId: Idx + From<usize>;
    fn successors(&self) -> &[Self::BasicBlockId];
}

/// The struct implementing this trait promises that the CFG contains two
/// sentinel nodes:
/// +   `entry_block` with id 0 must have no predecessors;
/// +   `exit_block` with id (`self.basic_blocks().len()-1`) must have no successors.
/// This invariant is checked by the method `validate`.
pub trait Cfg {
    type BasicBlock: BasicBlockWithSuccessors;
    fn basic_blocks(&self) -> &IndexVec<BasicBlockId<Self>, Self::BasicBlock>;
    fn entry_block(&self) -> BasicBlockId<Self> {
        0.into()
    }
    fn exit_block(&self) -> BasicBlockId<Self> {
        (self.basic_blocks().len() - 1).into()
    }
    /// Checks the invariant specified in the trait and panics if it does not
    /// hold.
    /// TODO: Move to validators module.
    fn validate(&self) {
        let basic_blocks = self.basic_blocks();
        assert!(
            basic_blocks.len() >= 2,
            "cfg must contain at least two basic blocks"
        );
        for block in basic_blocks {
            assert!(
                !block.successors().contains(&self.entry_block()),
                "the entry block must have no predecessors"
            );
        }
        assert!(
            basic_blocks[self.exit_block()].successors().is_empty(),
            "the exit block must have no successors"
        );
    }
    /// Returns an iterator that traverses the basic blocks in order that is
    /// guaranteed to visit a basic block only after all of its predecessors
    /// were visited.
    fn walk(&self) -> BasicBlockWalker<'_, Self::BasicBlock> {
        BasicBlockWalker::new(self.basic_blocks(), self.entry_block())
    }
    /// Returns an iterator that traverses the basic blocks in order that is
    /// guaranteed to visit a basic block only after all of its predecessors
    /// were visited.
    fn reverse_walk(&self) -> BasicBlockReverseWalker<'_, Self::BasicBlock> {
        BasicBlockReverseWalker::new(
            self.basic_blocks(),
            Cow::Owned(self.compute_predecessors()),
            self.exit_block(),
        )
    }
    /// Computes for each basic block its predecessor basic blocks.
    fn compute_predecessors(&self) -> IndexVec<BasicBlockId<Self>, Vec<BasicBlockId<Self>>> {
        let basic_blocks = self.basic_blocks();
        let mut predecessors = IndexVec::with_capacity(basic_blocks.len() + 1);
        predecessors.resize(basic_blocks.len() + 1, Vec::new());
        for (id, block) in self.basic_blocks().iter_enumerated() {
            for &successor in block.successors() {
                let vec = predecessors.get_mut(successor).unwrap();
                vec.push(id);
            }
        }
        predecessors
    }
}

type BasicBlockId<T> = <<T as Cfg>::BasicBlock as BasicBlockWithSuccessors>::BasicBlockId;

pub struct BasicBlockWalker<'a, BasicBlock>
where
    BasicBlock: BasicBlockWithSuccessors,
{
    basic_blocks: &'a IndexVec<BasicBlock::BasicBlockId, BasicBlock>,
    /// How many unvisited predecessors each basic block has?
    basic_block_predecessor_counts: IndexVec<BasicBlock::BasicBlockId, usize>,
    /// The queue of basic blocks ready to be visited.
    work_queue: VecDeque<BasicBlock::BasicBlockId>,
    /// How many basic blocks were already visited?
    visited_count: usize,
}

impl<'a, BasicBlock> BasicBlockWalker<'a, BasicBlock>
where
    BasicBlock: BasicBlockWithSuccessors,
{
    pub fn new(
        basic_blocks: &'a IndexVec<BasicBlock::BasicBlockId, BasicBlock>,
        entry_block: BasicBlock::BasicBlockId,
    ) -> Self {
        assert!(
            basic_blocks.len() >= 2,
            "the procedure must contain at least two basic blocks"
        );
        let mut basic_block_predecessor_counts = IndexVec::with_capacity(basic_blocks.len());
        basic_block_predecessor_counts.resize(basic_blocks.len(), 0);
        for block in basic_blocks {
            for successor in block.successors() {
                basic_block_predecessor_counts[*successor] += 1;
            }
        }
        for (id, &count) in basic_block_predecessor_counts.iter_enumerated() {
            eprintln!("block id={:?} count={}", id, count);
            if id == entry_block {
                assert_eq!(
                    count, 0,
                    "the entry block (id: {:?}) should have no predecessors",
                    entry_block
                );
            } else {
                assert!(count > 0, "unreachable block: {:?}", id);
            }
        }
        let mut work_queue = VecDeque::new();
        work_queue.push_back(entry_block);
        Self {
            basic_blocks,
            basic_block_predecessor_counts,
            work_queue,
            visited_count: 0,
        }
    }
}

impl<'a, BasicBlock> Iterator for BasicBlockWalker<'a, BasicBlock>
where
    BasicBlock: BasicBlockWithSuccessors,
{
    type Item = (BasicBlock::BasicBlockId, &'a BasicBlock);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.work_queue.pop_front() {
            let block = &self.basic_blocks[next_id];
            for &successor in block.successors() {
                let count = self
                    .basic_block_predecessor_counts
                    .get_mut(successor)
                    .unwrap();
                *count = count.checked_sub(1).unwrap();
                if *count == 0 {
                    self.work_queue.push_back(successor);
                }
            }
            self.visited_count += 1;
            Some((next_id, block))
        } else {
            assert_eq!(
                self.visited_count,
                self.basic_blocks.len(),
                "failed to visit all basic blocks"
            );
            None
        }
    }
}

pub struct BasicBlockReverseWalker<'a, BasicBlock>
where
    BasicBlock: BasicBlockWithSuccessors,
{
    basic_blocks: &'a IndexVec<BasicBlock::BasicBlockId, BasicBlock>,
    predecessors: Cow<'a, IndexVec<BasicBlock::BasicBlockId, Vec<BasicBlock::BasicBlockId>>>,
    /// How many unvisited successors each basic block has?
    basic_block_successor_counts: IndexVec<BasicBlock::BasicBlockId, usize>,
    /// The queue of basic blocks ready to be visited.
    work_queue: VecDeque<BasicBlock::BasicBlockId>,
    /// How many basic blocks were already visited?
    visited_count: usize,
}

impl<'a, BasicBlock> BasicBlockReverseWalker<'a, BasicBlock>
where
    BasicBlock: BasicBlockWithSuccessors,
{
    pub fn new(
        basic_blocks: &'a IndexVec<BasicBlock::BasicBlockId, BasicBlock>,
        predecessors: Cow<'a, IndexVec<BasicBlock::BasicBlockId, Vec<BasicBlock::BasicBlockId>>>,
        exit_block: BasicBlock::BasicBlockId,
    ) -> Self {
        assert!(
            basic_blocks.len() > 0,
            "the procedure must contain at least one basic block"
        );
        let mut basic_block_successor_counts = IndexVec::with_capacity(basic_blocks.len() + 1);
        basic_block_successor_counts.resize(basic_blocks.len() + 1, 0);
        for (id, block) in basic_blocks.iter_enumerated() {
            basic_block_successor_counts[id] = block.successors().len();
        }
        let mut work_queue = VecDeque::new();
        work_queue.push_back(exit_block);
        assert_eq!(predecessors.len(), basic_blocks.len() + 1);
        Self {
            basic_blocks,
            predecessors,
            basic_block_successor_counts,
            work_queue,
            visited_count: 0,
        }
    }
}

impl<'a, BasicBlock> Iterator for BasicBlockReverseWalker<'a, BasicBlock>
where
    BasicBlock: BasicBlockWithSuccessors,
{
    type Item = (BasicBlock::BasicBlockId, &'a BasicBlock);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_id) = self.work_queue.pop_front() {
            let block = &self.basic_blocks[next_id];
            for predecessor in &self.predecessors[next_id] {
                let count = self
                    .basic_block_successor_counts
                    .get_mut(*predecessor)
                    .unwrap();
                *count = count.checked_sub(1).unwrap();
                if *count == 0 {
                    self.work_queue.push_back(*predecessor);
                }
            }
            self.visited_count += 1;
            Some((next_id, block))
        } else {
            assert_eq!(
                self.visited_count,
                self.basic_blocks.len(),
                "failed to visit all basic blocks"
            );
            None
        }
    }
}
