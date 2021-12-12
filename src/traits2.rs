use std::fmt;
use std::fmt::{Formatter};

pub fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 { // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{}", p1); // can't print using the '{}' format specifier! does not implement fmt::display trait
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!
}

//can implment some common traits by derive PartialEq for equals
// debug for :? formatter

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

//can implement fmt::display por normal print

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}