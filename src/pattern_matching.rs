use std::collections::HashMap;

pub fn main() {

    /*
    let v = vec![0, 1, 2, 3];
    println!("{}", v[6]); // this will cause a panic!
    panic!("Farewell!"); */

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    let first = fruits.get(0);
    println!("{:?}", first);

    let third = fruits.get(2);
    println!("{:?}", third);

    let non_existent = fruits.get(99);
    let apple = fruits.get(1);

    println!("{:?}", non_existent);
    println!("{:p}", &apple);
    println!("{:p}", &"apple");
    
    match apple {
        Some(&"apple") => println!("xss"),
        Some(T) => println!("That's my lucky number!"),
        _ => println!("default"),
    }

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    let mut fruits2 = fruits.iter();

    println!("{}", fruits2.next().unwrap_or(&"strawberry"));
    println!("{:?}", fruits2.next());

    for (key, value) in fruits.iter().enumerate() {
        println!("el valor de la key es {} el valor en el vector es {}", key, value);
    }

    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's Some(7) my lucky number! pattern matching with if let");
    }

    let mut x: HashMap<&str, bool> = HashMap::new();

    x.insert(&"xx", true);
    x.insert(&"yy", true);
    x.insert(&"zz", true);

    for (u,v) in x {
        println!("el valor de la key es {} el valor en el Hashmap es {}", u, v);
    }

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }


    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let point = Point { x: 0, y: 7 };

        let Point { x, y } = point;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let numbers = (2, 4, 8, 16, 32);

    /// _ care with ownership _ still share it
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's Some(7) {:?} my lucky number! pattern matching with if let", a_number.unwrap());
    }
}