pub fn main() {
    let circle_declaration = Circle { radius: 2.0};
    let rectangle_declaration = Rectangle { height: 2.0, width: 5 as f64};

    println!("pointer to struct {:p}", &circle_declaration);
    print_area(&circle_declaration);
    print_area_dyn(&circle_declaration);
    print_area_generic(&circle_declaration);

    println!("pointer to struct {:p}", &rectangle_declaration);
    print_area(&rectangle_declaration);
    print_area_dyn(&rectangle_declaration);
    print_area_generic(&rectangle_declaration);

}

trait Area {
    fn calculate_area(&self) -> f64;
}

fn print_area(area: &impl Area) {
    println!("pointer function area {:p}", &area);
    println!("{}", area.calculate_area());
}

fn print_area_dyn(area: &dyn Area) {
    println!("pointer function dyn area {:p}", &area);
    println!("{}", area.calculate_area());
}

fn print_area_generic<T: Area>(area: &T) {
    println!("pointer function generic area {:p}", &area);
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
