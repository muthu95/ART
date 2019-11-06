use std;
use std::mem;

use crate::key_interface;

pub struct ArtNodeBase {
    pub num_children: u16,
    pub partial_len: usize,
    pub partial: [u8; 8],
}

impl ArtNodeBase {
    pub fn new() -> Self {
        ArtNodeBase {
            num_children: 0,
            partial_len: 0,
            partial: unsafe { mem::uninitialized() }
        }
    }

    pub fn compute_prefix_match<K: key_interface::KeyInterface>(&self, key: &K, depth: usize) -> usize {
        for i in 0..self.partial_len {
            if key.bytes()[i + depth] != self.partial[i] {
                return i;
            }
        }
        self.partial_len
    }
}
