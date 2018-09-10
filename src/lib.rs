use std::fmt;
mod aho_corasick;
mod common;
mod trie;

pub const CHAR_MAX: usize = 128;

/// Node index of trie/ac-automaton
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NodeId(pub(crate) u32);

impl NodeId {
    /// Represents no node
    pub const EMPTY: Self = NodeId(u32::max_value());
    /// Root state
    pub const ROOT: Self = NodeId(0);
    /// Get usize for slice indexing
    #[inline(always)]
    pub fn idx(self) -> usize {
        self.0 as usize
    }
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }
}

/// PatIdtern index of trie/ac-automaton
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PatId(pub(crate) u32);

impl PatId {
    /// Represents no pattern
    pub const EMPTY: Self = PatId(u32::max_value());
    /// Get usize for slice indexing
    #[inline(always)]
    pub fn idx(self) -> usize {
        self.0 as usize
    }
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }
}

/// Represents compressed order of ASCII bytes
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ByteOrd(pub(crate) u8);

impl ByteOrd {
    /// Represents this ascii byte is not available in the word set
    pub const EMPTY: Self = ByteOrd(u8::max_value());
    /// Get usize for slice indexing
    #[inline(always)]
    pub fn idx(self) -> usize {
        usize::from(self.0)
    }
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }
}

/// If given word set contains non-ascii character, this error is reported.
#[derive(Clone, Copy, Debug)]
pub struct InvalidByteError(char);

impl fmt::Display for InvalidByteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[const-trie] invalid byte: {}", self.0 as char)
    }
}

impl ::std::error::Error for InvalidByteError {}
