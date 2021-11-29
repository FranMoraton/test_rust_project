use std::collections::HashMap;
use std::slice::Iter;

fn main() {
    let mut x: HashMap<String, String> = HashMap::new();
    x.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    x.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    x.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    println!("{:?}", x.get("sss"));
    println!("{:?}", x.get("Programming in Rust"));

    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    x.remove(obsolete);
    x.remove("ddd");

    println!("\nReview for \'{}\': {:?}", obsolete, x.get(obsolete));

    let mut y: HashMap<u32, String> = HashMap::new();

    let mut order = 1;
    y.insert(order, String::from("xxx"));
    println!("{:?}", y.get(&order));
    println!("{:?}", y);

    order += 1;
    y.insert(order, String::from("yy"));
    println!("{:?}", y);
    println!("{:?}", y.get(&order));

    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };
    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);

    counter = 0;
    while counter < 5 {
        println!("We loop a while... {}", counter);
        counter = counter + 1;
    }

    for number in 0..5 {
        //5 es abierto
        println!("{}", number);
    }

    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds {
        println!("{}", bird);
    }

    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }

    let mut iterator = big_birds.iter();
    
    println!("The {} is a big bird.", iterator.next().unwrap());
    println!("The {:?} is a big bird.", iterator.next().ok_or("error"));
    println!("The {:?} is a big bird.", iterator.next().ok_or("error"));
    println!("The {:?} is a big bird.", iterator.next().ok_or("error"));
}
