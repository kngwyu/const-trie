use super::{ByteOrd, InvalidByteError, NodeId, PatId, CHAR_MAX};
use common;

#[derive(Clone)]
struct Node {
    next: [NodeId; CHAR_MAX],
    accept: PatId,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            next: [NodeId::EMPTY; CHAR_MAX],
            accept: PatId::EMPTY,
        }
    }
}

#[derive(Clone)]
struct TrieInner<P, T> {
    patterns: Vec<(P, T)>,
    nodes: Vec<Node>,
    initial_bytes: Vec<u8>,
    ord: [ByteOrd; CHAR_MAX],
    num_chars: usize,
}

impl<P: AsRef<[u8]>, T> TrieInner<P, T> {
    fn construct(words: impl Iterator<Item = (P, T)>) -> Result<Self, InvalidByteError> {
        let patterns: Vec<_> = words.collect();
        let (ord, num_chars) = common::ordering(patterns.iter().map(|t| &t.0))?;
        let initial_bytes = common::initial_bytes(patterns.iter().map(|t| &t.0))?;
        let mut nodes = vec![Node::default()];
        for (i, (s, _)) in patterns.iter().enumerate() {
            let mut cur = NodeId::ROOT;
            for &b in s.as_ref() {
                let ord = ord[b as usize];
                if nodes[cur.idx()].next[ord.idx()].is_empty() {
                    nodes[cur.idx()].next[ord.idx()] = NodeId(nodes.len() as u32);
                    nodes.push(Node::default());
                }
                cur = nodes[cur.idx()].next[ord.idx()];
            }
            nodes[cur.idx()].accept = PatId(i as u32);
        }
        Ok(TrieInner {
            patterns,
            nodes,
            initial_bytes,
            ord,
            num_chars,
        })
    }
    fn run(&self, pat: P) -> PatId {
        let bytes = pat.as_ref();
        let mut cur = NodeId::ROOT;
        if bytes.is_empty() || !self.initial_bytes.contains(&bytes[0]) {
            return PatId::EMPTY;
        }
        for &b in bytes {
            let ord = self.ord[b as usize];
            if ord.is_empty() {
                return PatId::EMPTY;
            }
            cur = self.nodes[cur.idx()].next[ord.idx()];
        }
        self.nodes[cur.idx()].accept
    }
}

#[derive(Clone)]
pub struct TrieSet<P> {
    inner: TrieInner<P, ()>,
}

impl<P: AsRef<[u8]>> TrieSet<P> {
    pub fn new(p: impl IntoIterator<Item = P>) -> Result<Self, InvalidByteError> {
        use std::iter;
        Ok(TrieSet {
            inner: TrieInner::construct(p.into_iter().zip(iter::repeat(())))?,
        })
    }
    pub fn contains(&self, pattern: P) -> bool {
        let pat_id = self.inner.run(pattern);
        pat_id != PatId::EMPTY
    }
}

#[derive(Clone)]
pub struct TrieMap<P, T> {
    inner: TrieInner<P, T>,
}

impl<P: AsRef<[u8]>, T> TrieMap<P, T> {
    pub fn new(p: impl IntoIterator<Item = (P, T)>) -> Result<Self, InvalidByteError> {
        Ok(TrieMap {
            inner: TrieInner::construct(p.into_iter())?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn set_test() {
        let test_data = &common::test_data::WORDS;
        let trie_set = TrieSet::new(test_data).unwrap();
        assert!(trie_set.contains(&test_data[3]));
        assert!(!trie_set.contains(&"ok"));
    }
    #[test]
    fn set_test_sparce() {
        let test_data = &common::test_data::WORDS_SPARCE;
        let trie_set = TrieSet::new(test_data).unwrap();
        assert!(trie_set.contains(&test_data[3]));
        assert!(!trie_set.contains(&"ok"));
        assert!(!trie_set.contains(&"aok"));
        assert!(!trie_set.contains(&"ab"));
    }
}
