use std;
use std::{mem, ptr};

use crate::key_interface;
use crate::art_node_base;
use crate::art_nodes;
use crate::art_node_interface;
use crate::node48;

macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = std::mem::uninitialized();
        for place in items.iter_mut() {
            std::ptr::write(place, $constructor);
        }
        items
    }}
}

pub struct NodeType256<K, V> {
    pub base_struct: art_node_base::ArtNodeBase,
    pub children: [art_nodes::ArtNodeEnum<K, V>; 256],
}


impl<K, V> NodeType256<K, V> {
    pub fn new() -> Self {
        NodeType256 {
            base_struct: art_node_base::ArtNodeBase::new(),
            children: unsafe {make_array!(256, art_nodes::ArtNodeEnum::Empty) }
        }
    }
}

impl<K: key_interface::KeyInterface, V> art_node_interface::ArtNodeInterface<K, V> for NodeType256<K, V> {
    fn add_child(&mut self, child: art_nodes::ArtNodeEnum<K, V>, byte: u8) {
        self.children[byte as usize] = child;
        self.base_struct.num_children += 1;
    }

    fn is_full(&self) -> bool {
        self.base_struct.num_children >= 256
    }

    fn to_art_node(self: Box<Self>) -> art_nodes::ArtNodeEnum<K,V> {
        art_nodes::ArtNodeEnum::Inner256(self)
    }

    fn grow_and_add(self, _leaf: art_nodes::ArtNodeEnum<K, V>, _byte: u8) -> art_nodes::ArtNodeEnum<K, V> {
        panic!("Cannot grow ArtNode256");
    }

    fn mut_base(&mut self) -> &mut art_node_base::ArtNodeBase {
        &mut self.base_struct
    }

    fn base(&self) -> &art_node_base::ArtNodeBase {
        &self.base_struct
    }

    fn find_child_mut(&mut self, byte: u8) -> Option<&mut art_nodes::ArtNodeEnum<K, V>> {
        match &self.children[byte as usize] {
            art_nodes::ArtNodeEnum::Empty => None,
            _ => Some(&mut self.children[byte as usize]),
        }
    }

    fn find_child(&self, byte: u8) -> Option<&art_nodes::ArtNodeEnum<K, V>> {
        match &self.children[byte as usize] {
            art_nodes::ArtNodeEnum::Empty => None,
            value => Some(&value),
        }
    }

    fn has_child(&self, byte: u8) -> bool {
        match &self.children[byte as usize] {
            art_nodes::ArtNodeEnum::Empty => false,
            _ => true,
        }
    }

    fn clean_child(&mut self, _byte: u8) -> bool {
        self.base_struct.num_children -= 1;
        self.base_struct.num_children <= 40
    }

    fn shrink(mut self) -> art_nodes::ArtNodeEnum<K,V> {
        // TODO: several lines here basically same for all the nodes
        //       try to dedupe somehow.
        //
        let mut new_node = Box::new(node48::NodeType48::new());
        new_node.base_struct.partial_len = self.base_struct.partial_len;

        unsafe {
            ptr::copy_nonoverlapping(
                self.base_struct.partial.as_ptr(),
                new_node.base_struct.partial.as_mut_ptr(),
                self.base_struct.partial.len());
        }

        for i in 0..256 {
            match mem::replace(&mut self.children[i], art_nodes::ArtNodeEnum::Empty) {
                art_nodes::ArtNodeEnum::Empty => continue,
                node => new_node.add_child(node, i as u8),
            }
        }

        art_nodes::ArtNodeEnum::Inner48(new_node)
    }

    fn get_minimum(&self) -> &art_nodes::ArtNodeEnum<K,V> {
        let idx = get_first_non_empty_cell(&self.children);
        &self.children[idx]
    }
}

fn get_first_non_empty_cell<K, V>(children_arr: &[art_nodes::ArtNodeEnum<K, V>; 256]) -> usize {
    let mut i: usize = 0;
    while i < 256 {
        match &children_arr[i] {
            art_nodes::ArtNodeEnum::Empty => i += 1,
            _ => break,
        }
    }
    i
}
