use crate::art_node_base;
use crate::art_nodes;

pub trait ArtNodeInterface<K, V> {
    fn add_child(&mut self, art_node_base: art_nodes::ArtNodeEnum<K, V>, byte: u8);
    fn grow_and_add(self, leaf: art_nodes::ArtNodeEnum<K, V>, byte: u8) -> art_nodes::ArtNodeEnum<K, V>;
    fn is_full(&self) -> bool;
    fn base(&self) -> &art_node_base::ArtNodeBase;
    fn mut_base(&mut self) -> &mut art_node_base::ArtNodeBase;
    fn to_art_node(self: Box<Self>) -> art_nodes::ArtNodeEnum<K, V>;
    fn find_child(&self, byte: u8) -> Option<&art_nodes::ArtNodeEnum<K, V>>;
    fn find_child_mut(&mut self, byte: u8) -> Option<&mut art_nodes::ArtNodeEnum<K, V>>;
    fn remove_child(self, byte: u8) -> art_nodes::ArtNodeEnum<K, V>;
    fn get_minimum(&self) -> &art_nodes::ArtNodeEnum<K, V>;
    fn replace_child(&mut self, byte: u8, art_node_base: art_nodes::ArtNodeEnum<K, V>);
}
