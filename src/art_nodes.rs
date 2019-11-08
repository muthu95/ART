use crate::key_interface;
use crate::node4;
use crate::node16;
use crate::node48;
use crate::node256;

pub enum ArtNodeEnum<K, V> {
    Empty,

    Inner4(Box<node4::NodeType4<K, V>>),
    Inner16(Box<node16::NodeType16<K, V>>),
    Inner48(Box<node48::NodeType48<K, V>>),
    Inner256(Box<node256::NodeType256<K, V>>),

    LeafNode(Box<(K,V)>),
}

impl<K: key_interface::KeyInterface, V> ArtNodeEnum<K, V> {
    pub fn key(&self) -> &K {
        match self {
            &ArtNodeEnum::LeafNode(ref ptr) => &ptr.0,
            _ => panic!("Not a leaf node. Hence Key not found"),
        }
    }

    pub fn value(self) -> V {
        match self {
            ArtNodeEnum::LeafNode(ptr) => ptr.1,
            _ => panic!("Not a leaf node. Hence Value not found"),
        }
    }

    pub fn create_leaf(key: K, value: V) -> ArtNodeEnum<K,V> {
        ArtNodeEnum::LeafNode(Box::new((key,value)))
    }
}
