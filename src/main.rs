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
    let s = String::from("randomString11");
    let p = String::from("randomString12");

    for i in 1..20 {
        let mut random_string = "randomString".to_string();
        let variable = i.to_string();
        random_string.push_str(&variable);
        art.insert_key(random_string.clone(), i);
        println!("Inserting string completed {}", random_string);
    }

    match art.get(&s) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }

    art.delete_key(&s);

    match art.get(&s) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }

    art.delete_key(&p);

    match art.get(&p) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }

}
