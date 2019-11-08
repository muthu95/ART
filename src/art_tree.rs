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

    fn split_node<N: ArtNodeInterface<K, V>>(
        mut ptr: Box<N>,
        prefix_match_len: usize,
        depth: usize,
        key: K,
        value: V,
    ) -> art_nodes::ArtNodeEnum<K, V> {
        let mut new_node = Box::new(node4::NodeType4::new());

        let next_byte_leaf = key.bytes()[depth + prefix_match_len];
        let next_byte_inner = ptr.base().partial[prefix_match_len];

        new_node.n.partial_len = prefix_match_len;

        unsafe {
            std::ptr::copy_nonoverlapping(
                ptr.base().partial.as_ptr(),
                new_node.n.partial.as_mut_ptr(),
                new_node.n.partial.len());

            let copy_len = ptr.base().partial_len - prefix_match_len - 1;
            let src = ptr.base().partial[prefix_match_len+1..ptr.base().partial_len].as_ptr();
            let dst = ptr.mut_base().partial[..copy_len].as_mut_ptr();
            std::ptr::copy(src, dst, copy_len);
        }

        ptr.mut_base().partial_len -= prefix_match_len + 1;

        new_node.add_child(ptr.to_art_node(), next_byte_inner);
        new_node.add_child(art_nodes::ArtNodeEnum::create_leaf(key, value), next_byte_leaf);

        art_nodes::ArtNodeEnum::Inner4(new_node)
    }

    fn internal_node_insert<N>(mut ptr: Box<N>, depth: usize, key: K, value: V) -> art_nodes::ArtNodeEnum<K, V>
        where N: ArtNodeInterface<K,V>
    {
        let prefix_match_len = ptr.base().compute_prefix_match(&key, depth);

        if prefix_match_len != ptr.base().partial_len {
            Self::split_node(ptr, prefix_match_len, depth, key, value)
        } else {
            let next_byte = key.bytes()[depth + prefix_match_len];

            if ptr.has_child(next_byte) {
                {
                    let child = ptr.find_child_mut(next_byte);
                    Self::insert_record(child, depth + prefix_match_len + 1, key, value);
                }
                ptr.to_art_node()
            } else if ptr.is_full() {
                ptr.grow_and_add(art_nodes::ArtNodeEnum::create_leaf(key, value), next_byte)
            } else {
                ptr.add_child(art_nodes::ArtNodeEnum::create_leaf(key, value), next_byte);
                ptr.to_art_node()
            }
        }
    }

    fn leaf_node_insert(lleaf: art_nodes::ArtNodeEnum<K,V>, key: K, value: V, depth: usize) -> art_nodes::ArtNodeEnum<K,V> {
        if *lleaf.key() == key {
            return art_nodes::ArtNodeEnum::create_leaf(key, value);
        }

        let mut new_node = Box::new(node4::NodeType4::new());

        let (lnext, rnext) = {
            let lkey = lleaf.key();

            let mut lcp = depth;
            let mut max_lcp = std::cmp::min(constants::PREFIX_LENGTH_LIMIT, key.bytes().len());
            max_lcp = std::cmp::min(max_lcp, lkey.bytes().len());

            while lcp < max_lcp && lkey.bytes()[lcp] == key.bytes()[lcp] {
                lcp += 1;
            }

            println!("lcp value is {}",lcp);
            if lcp > depth {
                unsafe {
                    std::ptr::copy(
                        key.bytes()[depth..].as_ptr(),
                        new_node.n.partial.as_mut_ptr(),
                        lcp - depth
                    );
                }
            }

            new_node.n.partial_len = lcp - depth;
            if lcp == max_lcp {
                lcp = lcp - 1;
            }

            if lcp >= lkey.bytes().len() {
                lcp = lkey.bytes().len() - 1;
            }

            if lcp >= key.bytes().len() {
                lcp = key.bytes().len() - 1;
            }
            (lkey.bytes()[lcp], key.bytes()[lcp])
        };

        let rleaf = art_nodes::ArtNodeEnum::create_leaf(key, value);
        new_node.add_child(lleaf, lnext);
        new_node.add_child(rleaf, rnext);
        art_nodes::ArtNodeEnum::Inner4(new_node)
    }

    fn insert_record(root: &mut art_nodes::ArtNodeEnum<K, V>, depth: usize, key: K, value: V) {
        *root = match mem::replace(root, art_nodes::ArtNodeEnum::Empty) {
            art_nodes::ArtNodeEnum::Empty => art_nodes::ArtNodeEnum::create_leaf(key, value),
            art_nodes::ArtNodeEnum::Inner4(ptr) => Self::internal_node_insert(ptr, depth, key, value),
            art_nodes::ArtNodeEnum::Inner16(ptr) => Self::internal_node_insert(ptr, depth, key, value),
            art_nodes::ArtNodeEnum::Inner48(ptr) => Self::internal_node_insert(ptr, depth, key, value),
            art_nodes::ArtNodeEnum::Inner256(ptr) => Self::internal_node_insert(ptr, depth, key, value),
            leaf => Self::leaf_node_insert(leaf, key, value, depth),
        };
    }

    pub fn insert_key(&mut self, key: K, value: V) {
        Self::insert_record(&mut self.root, 0, key, value);
        self.size += 1;
    }

    #[inline]
    fn search_inner<N: ArtNodeInterface<K,V>>(ptr: &'a N, key: &K, depth: usize) -> Option<&'a V> {
        let lcp = ptr.base().compute_prefix_match(key, depth);

        if lcp != ptr.base().partial_len {
            return None;
        }

        if let Some(ref child) = ptr.find_child(key.bytes()[depth + lcp]) {
            return Self::search_rec(child, key, depth + lcp + 1);
        }

        None
    }

    fn search_rec(root: &'a art_nodes::ArtNodeEnum<K,V>, key: &K, depth: usize) -> Option<&'a V> {
        match root {
            &art_nodes::ArtNodeEnum::Empty => None,
            &art_nodes::ArtNodeEnum::LeafNode(ref ptr) => if ptr.0 == *key {
                Some(&ptr.1)
            } else {
                None
            }
            &art_nodes::ArtNodeEnum::Inner4(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &art_nodes::ArtNodeEnum::Inner16(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &art_nodes::ArtNodeEnum::Inner48(ref ptr) => Self::search_inner(&**ptr, key, depth),

            &art_nodes::ArtNodeEnum::Inner256(ref ptr) => Self::search_inner(&**ptr, key, depth),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Self::search_rec(&self.root, key, 0)
    }

    fn leaf_node_delete(leaf: art_nodes::ArtNodeEnum<K,V>, key: &K) -> Option<V> {
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
        self.size -= 1;
        Self::delete_record(&mut self.root, 0, key)
    }
}
