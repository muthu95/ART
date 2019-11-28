use std;
use std::mem::MaybeUninit;
use crate::constants;

use crate::key_interface;

pub struct ArtNodeBase {
    pub num_children: u16,
    pub partial_len: usize,
    pub partial: [u8; constants::PREFIX_LENGTH_LIMIT],
}

impl ArtNodeBase {
    pub fn new() -> Self {
        ArtNodeBase {
            num_children: 0,
            partial_len: 0,
            partial: unsafe { MaybeUninit::uninit().assume_init() }
        }
    }

    pub fn compute_prefix_match<K: key_interface::KeyInterface>(&self, key: &K, depth: usize) -> usize {
        let limit = std::cmp::min(self.partial_len, constants::PREFIX_LENGTH_LIMIT);
        for i in 0..limit {
            if key.bytes()[i + depth] != self.partial[i] {
                return i;
            }
        }
        limit
    }
}
