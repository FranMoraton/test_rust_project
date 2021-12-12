pub fn main() {
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(Container::new(String::from("Bar")).value, String::from("Bar"));
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
    new_container(-1);
}

struct Container<T> {
    value: T,
}

fn new_container<T>(value: T) -> Container<T> {
    Container { value }
}

//declared in impl to set same type generic value for all methods in impl struct
impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}