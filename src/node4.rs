use std;
use std::{mem, ptr};
use crate::key_interface;
use crate::art_node_base;
use crate::art_nodes;
use crate::art_node_interface;

pub struct NodeType4<K, V> {
    pub n: art_node_base::ArtNodeBase,
    pub keys: mem::ManuallyDrop<[u8; 4]>,
    pub children: mem::ManuallyDrop<[art_nodes::ArtNodeEnum<K, V>; 4]>,
}

impl<K, V> NodeType4<K, V> {
    pub fn new() -> Self {
        NodeType4 {
            n: art_node_base::ArtNodeBase::new(),
            keys: unsafe { mem::uninitialized() },
            children: unsafe { mem::uninitialized() },
        }
    }
}

impl<K,V> Drop for NodeType4<K,V> {
    fn drop(&mut self) {
        for i in 0..self.n.num_children {
            drop(&mut self.children[i as usize]);
        }
    }
}

impl<K: key_interface::KeyInterface, V> art_node_interface::ArtNodeInterface<K, V> for NodeType4<K, V> {
    fn add_child(&mut self, child: art_nodes::ArtNodeEnum<K, V>, byte: u8) {
        let idx = self.n.num_children as usize;
        unsafe {
            ptr::write(&mut self.children[idx] as *mut art_nodes::ArtNodeEnum<K,V>, child);
            ptr::write(&mut self.keys[idx] as *mut u8, byte);
        }
        self.n.num_children += 1;
    }

    fn is_full(&self) -> bool {
        self.n.num_children >= 4
    }

    fn base(&self) -> &art_node_base::ArtNodeBase {
        &self.n
    }

    fn mut_base(&mut self) -> &mut art_node_base::ArtNodeBase {
        &mut self.n
    }

    fn to_art_node(self: Box<Self>) -> art_nodes::ArtNodeEnum<K,V> {
        art_nodes::ArtNodeEnum::Inner4(self)
    }

    fn has_child(&self, byte: u8) -> bool {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return true;
            }
        }
        false
    }

    fn find_child(&self, byte: u8) -> Option<&art_nodes::ArtNodeEnum<K, V>> {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return Some(&self.children[i as usize]);
            }
        }
        None
    }

    fn find_child_mut(&mut self, byte: u8) -> &mut art_nodes::ArtNodeEnum<K, V> {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return &mut self.children[i as usize];
            }
        }
        panic!("No requested child");
    }
}
