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
    let s = String::from("abcdefghijkl");
    let p = String::from("abcdefghijks");
    let q = String::from("abcdefghimno");

    for i in 100..990 {
        let mut random_string = "randomaaasdsdsdsfbdbvkdbvkstring".to_string();
        let variable = i.to_string();
        random_string.push_str(&variable);
        art.insert_key(random_string.clone(), i);
        println!("Inserting string completed {}", random_string);
    }

    for i in 100..1000 {
        let mut random_string = "randomaaasdsdsdsfbdbvkdbvkstring".to_string();
        let variable = i.to_string();
        random_string.push_str(&variable);
        println!("Searching {}", random_string);
        match art.get(&random_string) {
            Some(a) => println!("Recieved {}", a),
            None => println!("None val returned"),
        }
    }

    /*println!("Inserting {}", s);
    art.insert_key(s.clone(), 11);
    println!("Inserting {}", p);
    art.insert_key(p.clone(), 12);
    println!("Inserting {}", q);
    art.insert_key(q.clone(), 13);
    match art.get(&s) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }
    match art.get(&p) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }
    match art.get(&q) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }*/

    /*art.delete_key(&s);

    match art.get(&s) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }

    art.delete_key(&p);

    match art.get(&p) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }*/
}
