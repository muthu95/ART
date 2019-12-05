use std;
use std::{mem, ptr};
use crate::constants;
use crate::node256;
use crate::node16;
use crate::key_interface;
use crate::art_node_base;
use crate::art_nodes;
use crate::art_node_interface;

macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = std::mem::uninitialized();
        for place in items.iter_mut() {
            std::ptr::write(place, $constructor);
        }
        items
    }}
}

pub struct NodeType48<K, V> {
    pub base_struct: art_node_base::ArtNodeBase,
    pub keys: [u8; 256],
    pub children: mem::ManuallyDrop<[art_nodes::ArtNodeEnum<K, V>; 48]>,
}


impl<K, V> NodeType48<K, V> {
    pub fn new() -> Self {
        NodeType48 {
            base_struct: art_node_base::ArtNodeBase::new(),
            keys: [constants::EMPTY_CELL; 256],
            children: unsafe {mem::ManuallyDrop::new(make_array!(48, art_nodes::ArtNodeEnum::Empty))}
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
        let idx = get_first_empty_cell(&self.children);
        self.children[idx] = child;
        //unsafe { ptr::write(&mut self.children[idx] as *mut art_nodes::ArtNodeEnum<K,V>, child);}
        //+1 because indices in children arr is referred from [1, 48]. 0 is Empty cell.
        self.keys[byte as usize] = (idx+1) as u8;
        self.base_struct.num_children += 1;
    }

    fn is_full(&self) -> bool {
        self.base_struct.num_children >= 48
    }

    fn to_art_node(self: Box<Self>) -> art_nodes::ArtNodeEnum<K,V> {
        art_nodes::ArtNodeEnum::Inner48(self)
    }

    fn grow_and_add(mut self, leaf: art_nodes::ArtNodeEnum<K, V>, byte: u8) -> art_nodes::ArtNodeEnum<K, V> {
        println!("creating node256");
        let mut new_node = Box::new(node256::NodeType256::new());
        
        new_node.base_struct.partial_len = self.base_struct.partial_len;
        let mut i: usize = 0;
        while i < self.base_struct.partial_len && i < constants::PREFIX_LENGTH_LIMIT {
            new_node.base_struct.partial[i] = self.base_struct.partial[i];
            i += 1;
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
        &mut self.base_struct
    }

    fn base(&self) -> &art_node_base::ArtNodeBase {
        &self.base_struct
    }

    fn find_child_mut(&mut self, byte: u8) -> Option<&mut art_nodes::ArtNodeEnum<K, V>> {
        if self.keys[byte as usize] != constants::EMPTY_CELL {
            Some(&mut self.children[self.keys[byte as usize] as usize - 1])
        } else {
            None
        }
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

    fn remove_child(mut self, byte: u8) -> art_nodes::ArtNodeEnum<K, V> {
        let curr_children_count = self.base().num_children as usize;
        if curr_children_count == 17 {
            println!("Reducing node48 to node16");
            let mut new_node = Box::new(node16::NodeType16::new());
            new_node.mut_base().partial_len = self.base().partial_len;
            let mut i = 0;
            while i < self.base().partial.len() {
                new_node.mut_base().partial[i] = self.base().partial[i];
                i += 1;
            }
            i = 0;
            while i < 256 {
                if i as u8 != byte && self.keys[i as usize] != constants::EMPTY_CELL {
                    //println!("Moving {} vs {}", self.keys[i], byte);
                    let temp = mem::replace(&mut self.children[self.keys[byte as usize] as usize - 1], art_nodes::ArtNodeEnum::Empty);
                    new_node.add_child(temp, self.keys[i as usize]);
                }
                i += 1;
            }
            new_node.to_art_node()
        } else {
            self.children[self.keys[byte as usize] as usize - 1] = art_nodes::ArtNodeEnum::Empty;
            self.keys[byte as usize] = constants::EMPTY_CELL;
            self.base_struct.num_children -= 1;
            Box::new(self).to_art_node()
        }
    }

    fn replace_child(&mut self, byte: u8, child: art_nodes::ArtNodeEnum<K, V>) {
        self.children[self.keys[byte as usize] as usize - 1] = child;
    }

    fn shrink(mut self) -> art_nodes::ArtNodeEnum<K,V> {
        let mut new_node = Box::new(node16::NodeType16::new());
        new_node.base_struct.partial_len = self.base_struct.partial_len;

        unsafe {
            ptr::copy_nonoverlapping(
                self.base_struct.partial.as_ptr(),
                new_node.base_struct.partial.as_mut_ptr(),
                self.base_struct.partial.len());
        }

        for i in 0..256 {
            if self.keys[i] != constants::EMPTY_CELL {
                let child = std::mem::replace(&mut self.children[self.keys[i] as usize - 1], art_nodes::ArtNodeEnum::Empty);
                new_node.add_child(child, i as u8);
            }
        }

        art_nodes::ArtNodeEnum::Inner16(new_node)
    }

    fn get_minimum(&self) -> &art_nodes::ArtNodeEnum<K,V> {
        let idx = get_first_non_empty_cell(&self.children);
        &self.children[idx]
    }
}

fn get_first_empty_cell<K, V>(children_arr: &[art_nodes::ArtNodeEnum<K, V>; 48]) -> usize {
    let mut i: usize = 0;
    while i < 48 {
        if let art_nodes::ArtNodeEnum::Empty = children_arr[i] {
            break;
        }
        i += 1;
    }
    i
}

fn get_first_non_empty_cell<K, V>(children_arr: &[art_nodes::ArtNodeEnum<K, V>; 48]) -> usize {
    let mut i: usize = 0;
    while i < 48 {
        match &children_arr[i] {
            art_nodes::ArtNodeEnum::Empty => i += 1,
            _ => break,
        }
    }
    i
}
