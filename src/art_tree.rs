use std;
use std::mem;
use crate::key_interface;
use crate::constants;
use crate::node4;
use crate::art_nodes;
use crate::art_node_interface::ArtNodeInterface;


pub struct Art<K: key_interface::KeyInterface, V> {
    root: art_nodes::ArtNodeEnum<K, V>,
    size: usize,
}

impl<'a, K: 'a + key_interface::KeyInterface + std::cmp::PartialEq, V> Art<K, V> {
    pub fn new() -> Self {
        Art {
            root: art_nodes::ArtNodeEnum::Empty,
            size: 0,
        }
    }

    fn split_node<N: ArtNodeInterface<K,V>>(mut ptr: Box<N>, prefix_match_len: usize, depth: usize, key: K, value: V, existing_key_bytes: &[u8]) -> art_nodes::ArtNodeEnum<K, V> {
        //println!("****** {}", String::from_utf8_lossy(existing_key_bytes));
        let mut new_node = Box::new(node4::NodeType4::new());      
        new_node.mut_base().partial_len = prefix_match_len;
        let mut i: usize = 0;
        while i < prefix_match_len && i < constants::PREFIX_LENGTH_LIMIT {
            new_node.mut_base().partial[i] = existing_key_bytes[i];
            i += 1;
        }

        let next_byte_inner = existing_key_bytes[prefix_match_len];

        i = 0;
        while i < constants::PREFIX_LENGTH_LIMIT && (prefix_match_len + 1 + i) < ptr.base().partial_len {
            ptr.mut_base().partial[i] = existing_key_bytes[prefix_match_len + 1 + i];
            //println!("^^^ {}", existing_key_bytes[prefix_match_len + 1 + i] as char);
            i += 1;
        }
        ptr.mut_base().partial_len = ptr.base().partial_len - (prefix_match_len + 1);
        let target = std::cmp::min(ptr.base().partial_len, constants::PREFIX_LENGTH_LIMIT);
        println!("Split node, partialKey: {}", String::from_utf8_lossy(&ptr.base().partial[0..target]));
        //println!("Split node, partialKeyLen: {}", ptr.base().partial_len);
        
        let next_byte_leaf = key.bytes()[depth + prefix_match_len];
        new_node.add_child(art_nodes::ArtNodeEnum::create_leaf(key, value), next_byte_leaf);
        new_node.add_child(ptr.to_art_node(), next_byte_inner);
    
        art_nodes::ArtNodeEnum::Inner4(new_node)
    }

    fn internal_node_insert<N: ArtNodeInterface<K,V>>(mut ptr: Box<N>, mut depth: usize, key: K, value: V) -> art_nodes::ArtNodeEnum<K, V> {
        let prefix_match_len = ptr.base().compute_prefix_match(&key, depth);
        let actual_partial_len = ptr.base().partial_len;
        println!("INTERNAL INSERT, prefix_match_len: {}, actual_prefix_len: {}", prefix_match_len, actual_partial_len);
        let target = std::cmp::min(constants::PREFIX_LENGTH_LIMIT, actual_partial_len);
        if prefix_match_len != target {
            let extended_key = Self::find_minimum(&ptr).key();
            let extended_key_bytes = extended_key.bytes().to_vec();
            return Self::split_node(ptr, prefix_match_len, depth, key, value, &extended_key_bytes);
        } else if prefix_match_len == constants::PREFIX_LENGTH_LIMIT {
            let extended_key = Self::find_minimum(&ptr).key();
            let key_bytes = key.bytes();
            let extended_key_bytes = extended_key.bytes().to_vec();
            let mut lcp = depth + prefix_match_len;

            //Assuming keys are of same length
            while lcp < actual_partial_len && extended_key_bytes[lcp] == key_bytes[lcp] {
                lcp += 1;
            }
            println!("INTERNAL INSERT, extended prefix_match_len: {}", lcp);
            if lcp != actual_partial_len {
                return Self::split_node(ptr, lcp, depth, key, value, &extended_key_bytes);
            }
        }
        //Adding partial_len with assumption that keys are of same length.
        depth += actual_partial_len;
        let next_byte = key.bytes()[depth];
        match ptr.find_child_mut(next_byte) {
            Some(child) => {
                Self::insert_record(child, depth + 1, key, value);
                return ptr.to_art_node()
            },
            None => {
                if ptr.is_full() {
                    return ptr.grow_and_add(art_nodes::ArtNodeEnum::create_leaf(key, value), next_byte)
                } else {
                    ptr.add_child(art_nodes::ArtNodeEnum::create_leaf(key, value), next_byte);
                    return ptr.to_art_node()
                }
            },
        }
    }

    fn leaf_node_insert(leaf_node: art_nodes::ArtNodeEnum<K,V>, key: K, value: V, mut depth: usize) -> art_nodes::ArtNodeEnum<K,V> {
        if *leaf_node.key() == key {
            //If node with same key already exists, then update the value
            return art_nodes::ArtNodeEnum::create_leaf(key, value);
        }

        let mut new_node = Box::new(node4::NodeType4::new());
        let key2 = leaf_node.key();
        let mut lcp = depth;
        //Assuming keys are of same length.
        while lcp < key.bytes().len() && key2.bytes()[lcp] == key.bytes()[lcp] {
            if lcp - depth < constants::PREFIX_LENGTH_LIMIT {
                new_node.base_struct.partial[lcp-depth] = key.bytes()[lcp];
            }
            lcp += 1;
        }
        new_node.mut_base().partial_len = lcp - depth;
        //println!("lcp: {}, depth: {}, New node's partial_len: {}", lcp, depth, lcp-depth);
        let target = std::cmp::min(new_node.base().partial_len, constants::PREFIX_LENGTH_LIMIT);
        //println!("New node's partial key: {}", String::from_utf8_lossy(&new_node.base().partial[0..target]));
        depth += new_node.base().partial_len;
        
        let left_idx = key.bytes()[depth];
        let right_idx = key2.bytes()[depth];
        let new_leaf_node = art_nodes::ArtNodeEnum::create_leaf(key, value);
        new_node.add_child(new_leaf_node, left_idx);
        new_node.add_child(leaf_node, right_idx);
        art_nodes::ArtNodeEnum::Inner4(new_node)
    }

    fn insert_record(root: &mut art_nodes::ArtNodeEnum<K, V>, depth: usize, key: K, value: V) {
        *root = match mem::replace(root, art_nodes::ArtNodeEnum::Empty) {
            art_nodes::ArtNodeEnum::Empty => art_nodes::ArtNodeEnum::create_leaf(key, value),
            art_nodes::ArtNodeEnum::Inner4(ptr) => Self::internal_node_insert(ptr, depth, key, value),
            art_nodes::ArtNodeEnum::Inner16(ptr) => Self::internal_node_insert(ptr, depth, key, value),
            art_nodes::ArtNodeEnum::Inner48(ptr) => Self::internal_node_insert(ptr, depth, key, value),
            art_nodes::ArtNodeEnum::Inner256(ptr) => Self::internal_node_insert(ptr, depth, key, value),
            leaf_node => Self::leaf_node_insert(leaf_node, key, value, depth),
        };
    }

    pub fn insert_key(&mut self, key: K, value: V) {
        Self::insert_record(&mut self.root, 0, key, value);
        self.size += 1;
    }

    fn find_minimum<N: ArtNodeInterface<K,V>>(ptr: &Box<N>) -> &art_nodes::ArtNodeEnum<K,V> {
        let x = ptr.get_minimum();
        match x {
            art_nodes::ArtNodeEnum::Inner4(ref p) => Self::find_minimum(p),
            art_nodes::ArtNodeEnum::Inner16(ref p) => Self::find_minimum(p),
            art_nodes::ArtNodeEnum::Inner48(ref p) => Self::find_minimum(p),
            art_nodes::ArtNodeEnum::Inner256(ref p) => Self::find_minimum(p),
            _ => x,
        }
    }

    /*
    --------SEARCH-----------
    */
    #[inline]
    fn search_inner<N: ArtNodeInterface<K,V>>(ptr: &'a N, key: &K, mut depth: usize) -> Option<&'a V> {
        println!("SEARCH: partial_len: {}, depth: {}", ptr.base().partial_len, depth);

        let target = std::cmp::min(constants::PREFIX_LENGTH_LIMIT, ptr.base().partial_len);
        if ptr.base().compute_prefix_match(key, depth) != target {
            return None;
        }

        depth = depth + ptr.base().partial_len;
        match ptr.find_child(key.bytes()[depth]) {
            Some(child) => Self::search_rec(child, key, depth + 1),
            None => None,
        }
    }

    fn search_rec(root: &'a art_nodes::ArtNodeEnum<K,V>, key: &K, depth: usize) -> Option<&'a V> {
        match root {
            art_nodes::ArtNodeEnum::Empty => None,
            art_nodes::ArtNodeEnum::LeafNode(ref ptr) => {
                if ptr.0 == *key {
                    Some(&ptr.1)
                } else {
                    None
                }
            }
            art_nodes::ArtNodeEnum::Inner4(ref ptr) => Self::search_inner(&**ptr, key, depth),
            art_nodes::ArtNodeEnum::Inner16(ref ptr) => Self::search_inner(&**ptr, key, depth),
            art_nodes::ArtNodeEnum::Inner48(ref ptr) => Self::search_inner(&**ptr, key, depth),
            art_nodes::ArtNodeEnum::Inner256(ref ptr) => Self::search_inner(&**ptr, key, depth),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Self::search_rec(&self.root, key, 0)
    }

    /*fn leaf_node_delete(leaf: art_nodes::ArtNodeEnum<K,V>, key: &K) -> Option<V> {
        if *key == *leaf.key() {
            Some(leaf.value())
        } else {
            None
        }
    }

    fn internal_node_delete<N>(mut ptr: Box<N>, depth: usize, key: &K)-> (art_nodes::ArtNodeEnum<K,V>, Option<V>)
        where N: ArtNodeInterface<K,V>
    {
        let prefix_match_len = ptr.base().compute_prefix_match(key, depth);
        let next_byte = key.bytes()[depth + prefix_match_len];

        if prefix_match_len != ptr.base().partial_len || !ptr.has_child(next_byte) {
            (ptr.to_art_node(), None)
        } else {
            let ret = Self::delete_record(ptr.find_child_mut(next_byte), depth + prefix_match_len + 1, key);

            if let Some(&art_nodes::ArtNodeEnum::Empty) = ptr.find_child(next_byte) {
                // TODO: This is weird API, clean_child is called after the child has already been removed.
                //       Why does remove_child return should_shrink?
                //       Do this for now, but lets focus on this sometimes.
                //
                let should_shrink = ptr.clean_child(next_byte);

                if should_shrink {
                    // TODO: After shrink happens, we should recalculate partial
                    return (ptr.shrink(), ret);
                }
            }

            (ptr.to_art_node(), ret)
        }
    }

    fn delete_record(root: &mut art_nodes::ArtNodeEnum<K,V>, depth: usize, key: &K) -> Option<V> {
        let (new_root, ret) = match mem::replace(root, art_nodes::ArtNodeEnum::Empty) {
            art_nodes::ArtNodeEnum::Empty => (art_nodes::ArtNodeEnum::Empty, None),

            art_nodes::ArtNodeEnum::Inner4(ptr) => Self::internal_node_delete(ptr, depth, key),

            art_nodes::ArtNodeEnum::Inner16(ptr) => Self::internal_node_delete(ptr, depth, key),

            art_nodes::ArtNodeEnum::Inner48(ptr) => Self::internal_node_delete(ptr, depth, key),

            art_nodes::ArtNodeEnum::Inner256(ptr) => Self::internal_node_delete(ptr, depth, key),

            leaf => (art_nodes::ArtNodeEnum::Empty, Self::leaf_node_delete(leaf, key)),
        };

        *root = new_root;
        ret
    }

    pub fn delete_key(&mut self, key: &K) -> Option<V> {
        let del_result = Self::delete_record(&mut self.root, 0, key);
        if del_result.is_some() {
            self.size -= 1;
        }
        del_result
    }*/
}
