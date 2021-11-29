#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    mileage: Mileage,
}

#[derive(PartialEq, Debug)]
struct Mileage(Age, u32);

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> Mileage {
    let age: Age = if miles > 0 {
        Age::Used
    } else {
        Age::New
    };

    return Mileage(age, miles);
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    Car {
        color: color,
        motor: motor,
        roof: roof,
        mileage: car_quality(miles),
    }
}

fn main() {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut engine: Transmission = Transmission::Manual;
    let mut car: Car = car_factory(String::from(colors[2]), engine, true, 2);
    println!(
        "{:p} {:?} {:?} {:?}",
        &car, car.color, car.motor, car.mileage
    );

    let formal = true;
    let greeting = if formal {
        // if used here as an expression
        "Good day to you."
    } else {
        "Hey!"
    };
    println!("{}", greeting)
}
