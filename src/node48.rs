

use std;
use std::{mem, ptr};
use crate::constants;
use crate::node256;
use crate::key_interface;
use crate::art_node_base;
use crate::art_nodes;
use crate::art_node_interface;


pub struct NodeType48<K, V> {
    pub n: art_node_base::ArtNodeBase,
    pub keys: [u8; 256],
    pub children: mem::ManuallyDrop<[art_nodes::ArtNodeEnum<K, V>; 48]>,
}


impl<K, V> NodeType48<K, V> {
    pub fn new() -> Self {
        NodeType48 {
            n: art_node_base::ArtNodeBase::new(),
            keys: [constants::EMPTY_CELL; 256],
            children: unsafe { mem::uninitialized() }
        }
    }
}

impl<K,V> Drop for NodeType48<K,V> {
    fn drop(&mut self) {
        for i in 0..256 {
            if self.keys[i] != constants::EMPTY_CELL {
                drop(&mut self.children[self.keys[i] as usize - 1]);
            }
        }
    }
}

impl<K: key_interface::KeyInterface, V> art_node_interface::ArtNodeInterface<K, V> for NodeType48<K, V> {
    fn add_child(&mut self, child: art_nodes::ArtNodeEnum<K, V>, byte: u8) {
        unsafe {
            let idx = self.n.num_children as usize;
            ptr::write(&mut self.children[idx] as *mut art_nodes::ArtNodeEnum<K,V>, child);
        }
        self.n.num_children += 1;
        self.keys[byte as usize] = self.n.num_children as u8;
    }

    fn is_full(&self) -> bool {
        self.n.num_children >= 48
    }

    fn to_art_node(self: Box<Self>) -> art_nodes::ArtNodeEnum<K,V> {
        art_nodes::ArtNodeEnum::Inner48(self)
    }

    fn grow_and_add(mut self, leaf: art_nodes::ArtNodeEnum<K, V>, byte: u8) -> art_nodes::ArtNodeEnum<K, V> {
        let mut new_node = Box::new(node256::NodeType256::new());
        new_node.n.partial_len = self.n.partial_len;

        unsafe {
            ptr::copy_nonoverlapping(
                self.n.partial.as_ptr(),
                new_node.n.partial.as_mut_ptr(),
                self.n.partial.len());
        }

        new_node.add_child(leaf, byte);

        for i in 0..256 {
            if self.keys[i] != constants::EMPTY_CELL {
                let child = std::mem::replace(&mut self.children[self.keys[i] as usize - 1], art_nodes::ArtNodeEnum::Empty);
                new_node.add_child(child, i as u8);
            }
        }

        art_nodes::ArtNodeEnum::Inner256(new_node)
    }

    fn mut_base(&mut self) -> &mut art_node_base::ArtNodeBase {
        &mut self.n
    }

    fn base(&self) -> &art_node_base::ArtNodeBase {
        &self.n
    }

    fn find_child_mut(&mut self, byte: u8) -> &mut art_nodes::ArtNodeEnum<K, V> {
        &mut self.children[self.keys[byte as usize] as usize - 1]
    }

    fn find_child(&self, byte: u8) -> Option<&art_nodes::ArtNodeEnum<K, V>> {
        if self.keys[byte as usize] == constants::EMPTY_CELL {
            None
        } else {
            Some(&self.children[self.keys[byte as usize] as usize - 1])
        }
    }

    fn has_child(&self, byte: u8) -> bool {
        self.keys[byte as usize] != constants::EMPTY_CELL
    }
}
