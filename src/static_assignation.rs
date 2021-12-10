#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct Person2 {
    name: String,
    age: u8
}

pub fn main() {
    let x = String::from("hello there");
    let xslice: &str = &x[0..2];
    ss(xslice);
    println!("{:p}", &x);

}

fn ss(s: &str) {


    println!("{}", s);


    let name = "Peter";
    let age = 27;
    let peter = Person { name: s, age };
    let peter2 = Person2 { name: s.to_string(), age };

    // Pretty print
    println!("{:#?}", peter);

    println!("{:p}", peter.name);
    println!("{:p}", &peter2.name);
    println!("{:p}", s);
}