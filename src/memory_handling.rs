pub fn main() {
    // let mascot = String::from("ferris");
    // // mascot ownership was moved to ferris and mascot can no long be used, we could share the pointer
    // // let ferris = &mascot;
    // let ferris = mascot;
    // println!("{}", mascot);
    // println!("{}", ferris);
    //
    caller();
    caller_copy_trait();
    caller_clone();
}


fn process(input: String) {}

fn caller() {
    let s = String::from("Hello, world!");
    process(s); // Ownership of the string in `s` moved into `process`
    // process(s); // Error! ownership already moved.
    //a string do not implement copy trait so it moves the ownership to function and cant be processed again
}

fn process_copy_trait(input: u32) {}

fn caller_copy_trait() {
    let n = 1u32;
    process_copy_trait(n); // Ownership of the number in `n` copied into `process`
    process_copy_trait(n); // `n` can be used again because it wasn't moved, it was copied.
}

fn caller_clone() {
    let s = String::from("Hello, world!");
    process(s.clone()); // Ownership of the string in `s` moved into `process`
    // process(s); // Error! ownership already moved.
    //a string do not implement copy trait so it moves the ownership to function and cant be processed again
    // we can fix cloning the value ownership wont move from s var and can be processed later
    process(s);
}

