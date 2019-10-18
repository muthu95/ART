use crate::key_interface;
use crate::node4;

pub enum ArtNodeEnum<K, V> {
    Empty,

    Inner4(Box<node4::NodeType4<K, V>>),

    LeafNode(Box<(K,V)>),
}

impl<K: key_interface::KeyInterface, V> ArtNodeEnum<K, V> {
    pub fn key(&self) -> &K {
        match self {
            &ArtNodeEnum::LeafNode(ref ptr) => &ptr.0,
            _ => panic!("Does not contain key"),
        }
    }

    pub fn create_leaf(key: K, value: V) -> ArtNodeEnum<K,V> {
        ArtNodeEnum::LeafNode(Box::new((key,value)))
    }
}
