mod key_interface;
mod art_node_base;
mod node4;
mod node16;
mod node48;
mod node256;
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
    let s = String::from("randomString1000");
    let p = String::from("MuthuMM");

    for i in 1..1000 {
        let mut randomString = "randomString".to_string();
        let mut variable = i.to_string();
        randomString.push_str(&variable);
        art.insert(randomString.clone(), i);
        println!("Inserting string completed {}", randomString);
    }

    match art.get(&s) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }
}
