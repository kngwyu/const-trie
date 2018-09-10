use super::{ByteOrd, InvalidByteError, NodeId, PatId, CHAR_MAX};
use common;
use std::collections::VecDeque;

#[derive(Clone)]
struct Node {
    next: [NodeId; CHAR_MAX],
    fail: NodeId,
    accepts: Vec<PatId>,
}

impl Node {
    fn next(&self, ord: ByteOrd) -> NodeId {
        self.next[ord.idx()]
    }
    fn next_mut(&mut self, ord: ByteOrd) -> &mut NodeId {
        &mut self.next[ord.idx()]
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            next: [NodeId::EMPTY; CHAR_MAX],
            fail: NodeId::EMPTY,
            accepts: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct AcAutomaton<P> {
    patterns: Vec<P>,
    nodes: Vec<Node>,
    initial_bytes: Vec<u8>,
    ord: [ByteOrd; CHAR_MAX],
    max_ord: ByteOrd,
}

impl<P: AsRef<[u8]>> AcAutomaton<P> {
    pub fn construct(words: impl Iterator<Item = P>) -> Result<Self, InvalidByteError> {
        let patterns: Vec<_> = words.collect();
        let (ord, max_ord) = common::ordering(patterns.iter())?;
        let initial_bytes = common::initial_bytes(patterns.iter())?;
        let mut nodes = vec![Node::default()];
        for (i, s) in patterns.iter().enumerate() {
            let mut cur = NodeId::ROOT;
            for &b in s.as_ref() {
                let ord = ord[b as usize];
                if nodes[cur.idx()].next(ord).is_empty() {
                    *nodes[cur.idx()].next_mut(ord) = NodeId(nodes.len() as u32);
                    nodes.push(Node::default());
                }
                cur = nodes[cur.idx()].next(ord);
            }
            nodes[cur.idx()].accepts.push(PatId(i as u32));
        }
        let mut que = VecDeque::new();
        for ord in (0..max_ord.0).map(ByteOrd) {
            if nodes[NodeId::ROOT.idx()].next(ord).is_empty() {
                *nodes[NodeId::ROOT.idx()].next_mut(ord) = NodeId::ROOT;
            } else {
                let next = nodes[NodeId::ROOT.idx()].next(ord);
                nodes[next.idx()].fail = NodeId::ROOT;
                que.push_back(next);
            }
        }
        while let Some(cur) = que.pop_front() {
            for ord in (0..max_ord.0).map(ByteOrd) {
                if nodes[cur.idx()].next(ord).is_empty() {
                    continue;
                }
                let mut fail = nodes[cur.idx()].fail;
                while nodes[fail.idx()].next(ord).is_empty() {
                    fail = nodes[fail.idx()].fail;
                }
                nodes[cur.idx()].fail = nodes[fail.idx()].next(ord);
            }
        }
        Ok(AcAutomaton {
            patterns,
            nodes,
            initial_bytes,
            ord,
            max_ord,
        })
    }
    pub fn run(&self, query: &str) {
        let mut cur_idx = NodeId::ROOT;
        for &b in query.as_bytes() {
            let u = b as usize;
        }
    }
}

#[derive(Clone)]
pub struct Trie_<'a> {
    data: &'a [&'a str],
    transition: &'a [u32],
    ord: [u8; CHAR_MAX],
    num_chars: usize,
}
