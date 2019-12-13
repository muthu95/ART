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

use std::time::{Instant, Duration};

fn main() {
    println!("Hello, world!");
    short_string_test();
}

fn short_string_test() {
    let mut art: art_tree::Art<u32, u32> = art_tree::Art::new();
    for i in 0..1000000 {
        art.insert_key(i, i);
    }
    for i in 0..1000000 {
        art.get(&i);
    }
    let now = Instant::now();
    for i in 0..1000000 {
        art.delete_key(&i);
    }
    println!("{}", now.elapsed().as_micros());
}

/*let mut j:u8 = 0;
for i in 1..256 {
    let mut random_string = "abcd".to_string();
    let c = j as char;
    j += 1;
    random_string.push(c);
    art.insert_key(random_string.clone(), i);
    //println!("Inserting string completed {}", random_string);
}

j = 0;
for i in 1..256 {
    let mut random_string = "abcd".to_string();
    let c = j as char;
    random_string.push(c);
    println!("Deleting {}", j);
    art.delete_key(&random_string);
    j += 1;
}

j = 0;
for i in 1..256 {
    let mut random_string = "abcd".to_string();
    let c = j as char;
    random_string.push(c);
    //println!("Searching {}", random_string);
    match art.get(&random_string) {
        Some(a) => (),//println!("Recieved {}", a),
        None => println!("None val returned for {}", j),
    }
    j += 1;
}*/
