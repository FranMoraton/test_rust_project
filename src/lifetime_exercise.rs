// TODO: modify only this function.
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a str{
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

pub fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    );

    let integer_and_boolean = Point { x: 5, y: false, z: 2 };
    let float_and_string = Point { x: 1.0, y: "hey", z: 1.0 };
    let integer_and_float = Point { x: 5, y: 4.0, z: 2 };
    let both_integer = Point { x: 10, y: 30, z: 1 };
    let both_boolean = Point { x: true, y: true, z: true };
}

//two generics but x and z must be the same kind of value
struct Point<T, U> {
    x: T,
    y: U,
    z: T,
}
