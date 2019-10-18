mod key_interface;
mod art_node_base;
mod node4;
mod art_nodes;
mod constants;
mod art_node_interface;
mod art_tree;

fn main() {
    println!("Hello, world!");
    short_string_test();
}

fn short_string_test() {
    let mut art: art_tree::Art<String, u32> = art_tree::Art::new();
    let s = String::from("Raghavan");
    let p = String::from("Muthu");
    art.insert(s.clone(), 1);
    println!("Inserted key: {} with value: {}", s, 1);
    art.insert(p.clone(), 2);
    println!("Inserted key: {} with value: {}", p, 2);
}
