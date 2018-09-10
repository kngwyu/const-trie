use super::{ByteOrd, InvalidByteError, NodeId, PatId, CHAR_MAX};
use common;

#[derive(Clone)]
struct Node {
    next: [NodeId; CHAR_MAX],
    accept: PatId,
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
            accept: PatId::EMPTY,
        }
    }
}

#[derive(Clone)]
struct TrieInner<P, V> {
    patterns: Vec<(P, V)>,
    nodes: Vec<Node>,
    initial_bytes: Vec<u8>,
    ord: [ByteOrd; CHAR_MAX],
}

impl<P: AsRef<[u8]>, V> TrieInner<P, V> {
    fn construct(words: impl Iterator<Item = (P, V)>) -> Result<Self, InvalidByteError> {
        let patterns: Vec<_> = words.collect();
        let (ord, _) = common::ordering(patterns.iter().map(|t| &t.0))?;
        let initial_bytes = common::initial_bytes(patterns.iter().map(|t| &t.0))?;
        let mut nodes = vec![Node::default()];
        for (i, (s, _)) in patterns.iter().enumerate() {
            let mut cur = NodeId::ROOT;
            for &b in s.as_ref() {
                let ord = ord[b as usize];
                if nodes[cur.idx()].next(ord).is_empty() {
                    *nodes[cur.idx()].next_mut(ord) = NodeId(nodes.len() as u32);
                    nodes.push(Node::default());
                }
                cur = nodes[cur.idx()].next(ord);
            }
            nodes[cur.idx()].accept = PatId(i as u32);
        }
        Ok(TrieInner {
            patterns,
            nodes,
            initial_bytes,
            ord,
        })
    }
    fn run(&self, query: P) -> PatId {
        let query = query.as_ref();
        let mut cur = NodeId::ROOT;
        if query.is_empty() || !self.initial_bytes.contains(&query[0]) {
            return PatId::EMPTY;
        }
        for &b in query {
            if b as usize >= CHAR_MAX {
                return PatId::EMPTY;
            }
            let ord = self.ord[b as usize];
            if ord.is_empty() {
                return PatId::EMPTY;
            }
            cur = self.nodes[cur.idx()].next(ord);
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
pub struct TrieMap<P, V> {
    inner: TrieInner<P, V>,
}

impl<P: AsRef<[u8]>, V> TrieMap<P, V> {
    pub fn new(p: impl IntoIterator<Item = (P, V)>) -> Result<Self, InvalidByteError> {
        Ok(TrieMap {
            inner: TrieInner::construct(p.into_iter())?,
        })
    }
    pub fn get(&self, key: P) -> Option<&V> {
        let pat_id = self.inner.run(key);
        if !pat_id.is_empty() {
            Some(&self.inner.patterns[pat_id.idx()].1)
        } else {
            None
        }
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
        for i in 0..test_data.len() {
            assert!(trie_set.contains(&test_data[i]));
        }
        assert!(!trie_set.contains(&"ok"));
        assert!(!trie_set.contains(&"aok"));
        assert!(!trie_set.contains(&"ab"));
    }
    #[test]
    fn map_test() {
        let test_data = &common::test_data::WORDS;
        let trie_map = TrieMap::new(test_data.into_iter().enumerate().map(|t| (t.1, t.0))).unwrap();
        assert_eq!(trie_map.get(&test_data[3]), Some(&3));
        assert_eq!(trie_map.get(&"O Ye of Little Fai"), None);
    }
}
