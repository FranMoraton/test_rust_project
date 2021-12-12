pub fn main() {
    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

    call_struct_marked();
}
//we usually see this generic lifetime as <'a> but it can be whatever word you want,
// it only a mechanism to tell compiler how long the references will life
fn longest_word<'lifetime_associated_to_function>(
    x: &'lifetime_associated_to_function String,
    y: &'lifetime_associated_to_function String
) -> &'lifetime_associated_to_function String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//compiler expect references in longest_word to life the exact sametime while in this example
// magic2 reference will disappear in the scope, compiler wont let us do this even if magic1 is longer
// because compiler wont know in compile time
// fn example_breaking() {
//     let magic1 = String::from("abracadabra!");
//     let result;
//     {
//         let magic2 = String::from("shazam!");
//         result = longest_word(&magic1, &magic2);
//     }
//     println!("The longest magic word is {}", result);
// }

#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: String) { }

fn call_struct_marked() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    // we cant move text its borrowed to fox and dog
    erase(text);

    println!("{:?}", fox);
    println!("{:?}", dog);
}

