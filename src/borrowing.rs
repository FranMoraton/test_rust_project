pub fn main() {
    let greeting = String::from("hello");
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`

    greeting_main();
    change_main_mutable();

    // cant borrow twice
    // Your code must implement either of the following definitions, but not both at the same time:
    //
    // One or more immutable references (&T)
    // Exactly one mutable reference (&mut T)
    // let mut value = String::from("hello");
    //
    // let ref1 = &mut value;
    // let ref2 = &mut value;
    //
    // println!("{}, {}", ref1, ref2);


    //    error[E0502]: cannot borrow `value` as mutable because it is also borrowed as immutable
    //      --> src/main.rs:5:16
    // let mut value = String::from("hello");
    //
    // let ref1 = &value;
    // let ref2 = &mut value;
    //
    // println!("{}, {}", ref1, ref2);
}

fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn greeting_main() {
    let greeting = String::from("Hello");
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
}

// fn change(message: &String) {
//     message.push_str("!"); // We try to add a "!" to the end of our message
// }
//
// fn change_main() {
//     let greeting = String::from("Hello");
//     change(&greeting);
// }
// cant mutate a value borrowed if inmutable

fn change_main_mutable() {
    let mut greeting = String::from("hello");
    change(&mut greeting);
    println!("{}", greeting);
}

fn change(text: &mut String) {
    text.push_str(", world");
}