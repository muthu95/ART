use std;
use std::{mem};
use std::mem::MaybeUninit;
use crate::node16;
use crate::key_interface;
use crate::art_node_base;
use crate::art_nodes;
use crate::art_node_interface;
use crate::constants;

macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = std::mem::uninitialized();
        for place in items.iter_mut() {
            std::ptr::write(place, $constructor);
        }
        items
    }}
}

pub struct NodeType4<K, V> {
    pub base_struct: art_node_base::ArtNodeBase,
    pub keys: mem::ManuallyDrop<[u8; 4]>,
    pub children: mem::ManuallyDrop<[art_nodes::ArtNodeEnum<K, V>; 4]>,
}

impl<K, V> NodeType4<K, V> {
    pub fn new() -> Self {
        NodeType4 {
            base_struct: art_node_base::ArtNodeBase::new(),
            keys: unsafe { MaybeUninit::uninit().assume_init() },
            children: unsafe {mem::ManuallyDrop::new(make_array!(4, art_nodes::ArtNodeEnum::Empty))},
        }
    }
}

impl<K,V> Drop for NodeType4<K,V> {
    fn drop(&mut self) {
        for i in 0..self.base_struct.num_children {
            drop(&mut self.children[i as usize]);
        }
    }
}

impl<K: key_interface::KeyInterface, V> art_node_interface::ArtNodeInterface<K, V> for NodeType4<K, V> {
    fn add_child(&mut self, child: art_nodes::ArtNodeEnum<K, V>, byte: u8) {
        let idx = get_sorted_index(byte, &self.keys, self.base_struct.num_children);
        //println!("Pos in sorted order is: {}", idx);

        //Shifting elements to the right by 1
        let mut i = self.base_struct.num_children as usize;
        while i > idx {
            let temp = mem::replace(&mut self.children[i-1], art_nodes::ArtNodeEnum::Empty);
            self.children[i] = temp;
            self.keys[i] = self.keys[i-1];
            i -= 1;
        }

        //Adding the key & child in correct sorted position
        self.keys[idx] = byte;
        self.children[idx] = child;
        //unsafe { ptr::write(&mut self.children[idx] as *mut art_nodes::ArtNodeEnum<K,V>, child); }
        
        self.base_struct.num_children += 1;
    }

    fn grow_and_add(mut self, leaf: art_nodes::ArtNodeEnum<K, V>, byte: u8) -> art_nodes::ArtNodeEnum<K, V> {
        println!("creating node16");
        let mut new_node = Box::new(node16::NodeType16::new());

        new_node.mut_base().partial_len = self.base_struct.partial_len;
        let mut i: usize = 0;
        while i < self.base_struct.partial_len && i < constants::PREFIX_LENGTH_LIMIT {
            new_node.mut_base().partial[i] = self.base().partial[i];
            i += 1;
        } 

        for i in 0..4 {
            let child = std::mem::replace(&mut self.children[i], art_nodes::ArtNodeEnum::Empty);
            new_node.add_child(child, self.keys[i]);
        }
        new_node.add_child(leaf, byte);
        art_nodes::ArtNodeEnum::Inner16(new_node)
    }

    fn find_child(&self, byte: u8) -> Option<&art_nodes::ArtNodeEnum<K, V>> {
        for i in 0..self.base_struct.num_children {
            if self.keys[i as usize] == byte {
                return Some(&self.children[i as usize]);
            }
        }
        None
    }

    fn find_child_mut(&mut self, byte: u8) -> Option<&mut art_nodes::ArtNodeEnum<K, V>> {
        for i in 0..self.base_struct.num_children {
            if self.keys[i as usize] == byte {
                return Some(&mut self.children[i as usize]);
            }
        }
        None
    }

    fn is_full(&self) -> bool {
        self.base_struct.num_children >= 4
    }

    fn base(&self) -> &art_node_base::ArtNodeBase {
        &self.base_struct
    }

    fn mut_base(&mut self) -> &mut art_node_base::ArtNodeBase {
        &mut self.base_struct
    }

    fn to_art_node(self: Box<Self>) -> art_nodes::ArtNodeEnum<K,V> {
        art_nodes::ArtNodeEnum::Inner4(self)
    }

    fn has_child(&self, byte: u8) -> bool {
        for i in 0..self.base_struct.num_children {
            if self.keys[i as usize] == byte {
                return true;
            }
        }
        false
    }

    //Assumption that child exists
    fn remove_child(mut self: Self, byte: u8) -> art_nodes::ArtNodeEnum<K, V> {
        let mut i = 0;
        let curr_children_count = self.base().num_children;
        /*if curr_children_count == 2 {
            println!("Reducing node4 to leaf");
            while i < curr_children_count {
                if self.keys[i as usize] != byte {
                    //Returning leaf
                    return mem::replace(&mut self.children[i as usize], art_nodes::ArtNodeEnum::Empty);
                }
                i += 1;
            }
            art_nodes::ArtNodeEnum::Empty
        } else {*/
            while i < curr_children_count {
                if self.keys[i as usize] == byte {
                    break;
                }
                i += 1;
            }
            //println!("%%% {}", i);
            while i < (curr_children_count - 1) {
                self.keys[i as usize] = self.keys[(i+1) as usize];
                let temp = mem::replace(&mut self.children[(i+1) as usize], art_nodes::ArtNodeEnum::Empty);
                self.children[i as usize] = temp;
                i += 1;
            }
            //println!("MOVED");
            self.children[(curr_children_count - 1) as usize] = art_nodes::ArtNodeEnum::Empty;
            //println!("DELETED");
            self.mut_base().num_children -= 1;
            //println!("Children count = {}", self.base().num_children);
            Box::new(self).to_art_node()
        //}
    }

    fn replace_child(&mut self, byte: u8, child: art_nodes::ArtNodeEnum<K, V>) {
        let mut i = 0;
        let curr_children_count = self.base().num_children;
        while i < curr_children_count as usize {
            if self.keys[i] == byte {
                self.children[i] = child;
                break;
            }
            i += 1;
        }
    }

    fn shrink(self) -> art_nodes::ArtNodeEnum<K,V> {
        art_nodes::ArtNodeEnum::Empty
    }

    fn get_minimum(&self) -> &art_nodes::ArtNodeEnum<K,V> {
        &self.children[0]
    }
}

fn get_sorted_index(byte: u8, byte_arr: &[u8; 4], num_children: u16) -> usize {
    let mut i:usize = 0;
    while i < num_children as usize && byte > byte_arr[i] {
        i += 1;
    }
    i
}
