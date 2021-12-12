pub fn main() {
    let circle_declaration = Circle { radius: 2.0};
    let rectangle_declaration = Rectangle { height: 2.0, width: 5 as f64};

    print_area(&circle_declaration);
    print_area(&rectangle_declaration);
}

trait Area {
    fn calculate_area(&self) -> f64;
}

fn print_area(area: &dyn Area) {
    println!("{}", area.calculate_area());
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * &self.radius.powf(2.0)
    }
}

impl Area for Rectangle {
    fn calculate_area(&self) -> f64 {
        &self.height * &self.width
    }
}
