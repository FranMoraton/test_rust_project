fn main() {
    let mut a_number = 10;
    println!("Hello, world {}!", a_number);
    a_number = 15;
    println!("Hello, world {}!", a_number);

    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num = shadow_num + 5;

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2;

    println!("The number is {}.", shadow_num);

    let number: u32 = 14;
    println!("The number is {}.", number);

    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);
    
    // Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool }    

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    // Unit struct
    struct Unit;

    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);


    let user_3 = Student { name: "Dyson X".to_string(), level: 5, remote: false };
    println!("{:#?} {} {}", user_3.name, user_3.level, user_3.remote);

    let x : String = "23232323".to_string();
    let y : String = String::from("23232323");
    let z : &str = "2323232";
    let mut output: Vec<char> = Vec::new();
    output.push('2');

    let v = vec!(1,2,3,4);
    println!("{:?}", output);
    println!("{:?}", v);
    println!("{:p}", &output);
    println!("{:p}", &v);
    println!("{:?}", std::mem::size_of_val(&output));
    println!("{:?}", std::mem::size_of_val(&v));
}
