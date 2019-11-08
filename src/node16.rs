
use std;
use std::{mem, ptr};
use crate::node48;
use crate::node4;
use crate::key_interface;
use crate::art_node_base;
use crate::art_nodes;
use crate::art_node_interface;
use crate::constants;

pub struct NodeType16<K, V> {
    pub n: art_node_base::ArtNodeBase,
    pub keys: mem::ManuallyDrop<[u8; 16]>,
    pub children: mem::ManuallyDrop<[art_nodes::ArtNodeEnum<K, V>; 16]>,
}


impl<K, V> NodeType16<K, V> {
    pub fn new() -> Self {
        NodeType16 {
            n: art_node_base::ArtNodeBase::new(),
            keys: unsafe { mem::uninitialized() },
            children: unsafe { mem::uninitialized() }
        }
    }
}

impl<K,V> Drop for NodeType16<K,V> {
    fn drop(&mut self) {
        for i in 0..self.n.num_children {
            drop(&mut self.children[i as usize]);
        }
    }
}

impl<K: key_interface::KeyInterface, V> art_node_interface::ArtNodeInterface<K, V> for NodeType16<K, V> {
    fn add_child(&mut self, child: art_nodes::ArtNodeEnum<K, V>, byte: u8) {
        let idx = self.n.num_children as usize;
        unsafe {
            ptr::write(&mut self.children[idx] as *mut art_nodes::ArtNodeEnum<K,V>, child);
            ptr::write(&mut self.keys[idx] as *mut u8, byte);
        }
        self.n.num_children += 1;
    }

    fn grow_and_add(mut self, leaf: art_nodes::ArtNodeEnum<K, V>, byte: u8) -> art_nodes::ArtNodeEnum<K, V> {
        let mut new_node = Box::new(node48::NodeType48::new());
        println!("creating node48");
        new_node.n.partial_len = self.n.partial_len;

        unsafe {
            ptr::copy_nonoverlapping(
                self.n.partial.as_ptr(),
                new_node.n.partial.as_mut_ptr(),
                self.n.partial.len());
        }

        new_node.add_child(leaf, byte);

        for i in 0..16 {
            let child = std::mem::replace(&mut self.children[i], art_nodes::ArtNodeEnum::Empty);
            new_node.add_child(child, self.keys[i]);
        }

        art_nodes::ArtNodeEnum::Inner48(new_node)
    }

    fn is_full(&self) -> bool {
        self.n.num_children >= 16
    }

    fn to_art_node(self: Box<Self>) -> art_nodes::ArtNodeEnum<K,V> {
        art_nodes::ArtNodeEnum::Inner16(self)
    }

    fn mut_base(&mut self) -> &mut art_node_base::ArtNodeBase {
        &mut self.n
    }

    fn base(&self) -> &art_node_base::ArtNodeBase {
        &self.n
    }

    fn find_child_mut(&mut self, byte: u8) -> &mut art_nodes::ArtNodeEnum<K, V> {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return &mut self.children[i as usize];
            }
        }
        panic!("No requested child");
    }

    fn find_child(&self, byte: u8) -> Option<&art_nodes::ArtNodeEnum<K, V>> {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return Some(&self.children[i as usize]);
            }
        }
        None
    }

    fn has_child(&self, byte: u8) -> bool {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                return true;
            }
        }
        false
    }

    fn clean_child(&mut self, byte: u8) -> bool {
        for i in 0..self.n.num_children {
            if self.keys[i as usize] == byte {
                self.keys[i as usize] = constants::EMPTY_CELL;
                self.n.num_children -= 1;

                self.children.swap(i as usize, self.n.num_children as usize);
                self.keys.swap(i as usize, self.n.num_children as usize);

                return self.n.num_children <= 2
            }
        }
        panic!("Removing child not found");
    }

    fn shrink(mut self) -> art_nodes::ArtNodeEnum<K,V> {
        let mut new_node = Box::new(node4::NodeType4::new());

        new_node.n.partial_len = self.n.partial_len;

        unsafe {
            ptr::copy_nonoverlapping(
                self.n.partial.as_ptr(),
                new_node.n.partial.as_mut_ptr(),
                self.n.partial.len());
        }

        for i in 0..self.n.num_children {
            let child = std::mem::replace(&mut self.children[i as usize], art_nodes::ArtNodeEnum::Empty);
            new_node.add_child(child, self.keys[i as usize]);
        }

        art_nodes::ArtNodeEnum::Inner4(new_node)
    }
}
