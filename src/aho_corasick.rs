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
    pub fn construct(words: impl IntoIterator<Item = P>) -> Result<Self, InvalidByteError> {
        let patterns: Vec<_> = words.into_iter().collect();
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
        // 2. create failure links
        let mut que = VecDeque::new();
        common::transitions(max_ord).for_each(|ord| {
            if nodes[NodeId::ROOT.idx()].next(ord).is_empty() {
                *nodes[NodeId::ROOT.idx()].next_mut(ord) = NodeId::ROOT;
            } else {
                let next = nodes[NodeId::ROOT.idx()].next(ord);
                nodes[next.idx()].fail = NodeId::ROOT;
                que.push_back(next);
            }
        });
        while let Some(cur) = que.pop_front() {
            for ord in common::transitions(max_ord) {
                let nxt = nodes[cur.idx()].next(ord);
                if nxt.is_empty() {
                    continue;
                }
                que.push_back(nxt);
                let mut cfail = nodes[cur.idx()].fail;
                while nodes[cfail.idx()].next(ord).is_empty() {
                    cfail = nodes[cfail.idx()].fail;
                }
                let (nxt, cfail) = common::get_two(&mut nodes, nxt.idx(), cfail.idx());
                nxt.fail = cfail.next(ord);
                nxt.accepts.extend_from_slice(&cfail.accepts);
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
    pub fn run(&self, query: P) -> Vec<PatId> {
        let query = query.as_ref();
        let mut cur = &self.nodes[NodeId::ROOT.idx()];
        let mut out = Vec::new();
        for &b in query {
            if b as usize >= CHAR_MAX {
                cur = &self.nodes[NodeId::ROOT.idx()];
                continue;
            }
            let ord = self.ord[b as usize];
            if ord.is_empty() {
                cur = &self.nodes[NodeId::ROOT.idx()];
                continue;
            }
            // make transition
            while cur.next(ord).is_empty() {
                cur = &self.nodes[cur.fail.idx()];
            }
            cur = &self.nodes[cur.next(ord).idx()];
            out.extend_from_slice(&cur.accepts);
        }
        out
    }
    pub fn get_pat(&self, id: PatId) -> &P {
        &self.patterns[id.idx()]
    }
}

#[derive(Clone)]
pub struct Trie_<'a> {
    data: &'a [&'a str],
    transition: &'a [u32],
    ord: [u8; CHAR_MAX],
    num_chars: usize,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn contains_test() {
        let test_data = &common::test_data::WORDS;
        let ac = AcAutomaton::construct(test_data).unwrap();
        let query = "Neon Pretty Basilisk Maggie Moneyeyes";
        let pats = ac.run(&query);
        assert!(
            pats.iter()
                .find(|&&id| **ac.get_pat(id) == "Neon")
                .is_some()
        );
        assert!(
            pats.iter()
                .find(|&&id| **ac.get_pat(id) == "Neno")
                .is_none()
        );
    }
}
