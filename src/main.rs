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

    /*println!("Inserting {}", s);
    art.insert_key(s.clone(), 11);
    match art.get(&s) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }
    println!("Deleting {}", s);
    art.delete_key(&s);
    match art.get(&s) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned"),
    }*/
    let mut j:u8 = 0;
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
    }

    /*println!("Deleting randomaaasdsdsdsfbdbvkdbvkstring13");
    art.delete_key(&String::from("randomaaasdsdsdsfbdbvkdbvkstring13"));
    println!("Deleting randomaaasdsdsdsfbdbvkdbvkstring11");
    art.delete_key(&String::from("randomaaasdsdsdsfbdbvkdbvkstring11"));
    println!("Deleting randomaaasdsdsdsfbdbvkdbvkstring100");
    art.delete_key(&String::from("randomaaasdsdsdsfbdbvkdbvkstring100"));*/
    /*let mut i = 100;
    while i < 1000 {
        let mut random_string = "randomaaasdsdsdsfbdbvkdbvkstring".to_string();
        let variable = i.to_string();
        random_string.push_str(&variable);
        println!("Deleting {}", random_string);
        art.delete_key(&random_string);
        i += 2;
    }*/

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

/*
for i in 100..1000 {
        let mut random_string = "randomaaasdsdsdsfbdbvkdbvkstring".to_string();
        let variable = i.to_string();
        random_string.push_str(&variable);
        art.insert_key(random_string.clone(), i);
        println!("Inserting string completed {}", random_string);
    }
    let mut i = 2560;
    while i < 1000 {
        let mut random_string = "randomaaasdsdsdsfbdbvkdbvkstring".to_string();
        let variable = i.to_string();
        random_string.push_str(&variable);
        println!("Deleting {}", random_string);
        art.delete_key(&random_string);
        i += 1;
        if i%50 == 0 {
            i += 50;
        }
    }
    let mut i = 2560;
    while i < 1000 {
        let mut random_string = "randomaaasdsdsdsfbdbvkdbvkstring".to_string();
        let variable = i.to_string();
        random_string.push_str(&variable);
        println!("Deleting {}", random_string);
        art.delete_key(&random_string);
        i += 1;
        if i%50 == 0 {
            i += 50;
        }
    }

    for i in 100..1000 {
        let mut random_string = "randomaaasdsdsdsfbdbvkdbvkstring".to_string();
        let variable = i.to_string();
        random_string.push_str(&variable);
        //println!("Searching {}", random_string);
        match art.get(&random_string) {
            Some(a) => (),//println!("Recieved {}", a),
            None => println!("None val returned for {}", random_string),
        }
    }*/

/* 
let mut j:u8 = 0;
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
    j += 1;
    random_string.push(c);
    //println!("Searching {}", random_string);
    match art.get(&random_string) {
        Some(a) => println!("Recieved {}", a),
        None => println!("None val returned for {}", random_string),
    }
}

let mut i = 100;
while i < 1000 {
    let mut random_string = "randomaaasdsdsdsfbdbvkdbvkstring".to_string();
    let variable = i.to_string();
    random_string.push_str(&variable);
    println!("Deleting {}", random_string);
    art.delete_key(&random_string);
    i += 2;
}
*/
