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
      
      
          let days = ["ss", "tt"];
          let mut days_empty=["ss";7]; // defines array first element and length
          days_empty[1] = "tt";
          days_empty[2] = "tt";
          days_empty[3] = "tt";
          days_empty[4] = "tt";
          println!("{:?}", days_empty);
      
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
      
          // Instantiate a MouseClick struct and bind the coordinate values
          let click = MouseClick { x: 100, y: 250 };
          println!("Mouse click location: {}, {}", click.x, click.y);
              
          // Instantiate a KeyPress tuple and bind the key values
          let keys = KeyPress(String::from("Ctrl+"), 'N');
          println!("\nKeys pressed: {}{}", keys.0, keys.1);
              
          // Instantiate WebEvent enum variants
          // Set the boolean page Load value to true
          let we_load = WebEvent::WELoad;
          // Set the WEClick variant to use the data in the click struct
          let we_click = WebEvent::WEClick(click);
          // Set the WEKeys variant to use the data in the keys tuple
          let we_key = WebEvent::WEKeys(keys);
              
          // Print the values in the WebEvent enum variants
          // Use the {:#?} syntax to display the enum structure and data in a readable form
          println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
      
          let num = 25;
          println!("{} divided by 5 = {}", num, divide_by_5(num));
      
          // Order three cars
          let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
          println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
          
          car = car_factory(String::from("Silver"), Transmission::Automatic, true);
          println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    
          
          car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
          println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);
      }
      
      //derive debug macro allows to print structs
      #[derive(Debug)]
      enum WebEvent {
          // An enum variant can be like a unit struct without fields or data types
          WELoad,
          // An enum variant can be like a tuple struct with data types but no named fields
          WEKeys(KeyPress),
          // An enum variant can be like a classic struct with named fields and their data types
          WEClick(MouseClick)
      }
      
      // Define a tuple struct
      #[derive(Debug)]
      struct KeyPress(String, char);
      
      // Define a classic struct
      #[derive(Debug)]
      struct MouseClick { x: i64, y: i64 }
      
      fn divide_by_5(num: u32) -> u32 {
          if 0 == num {
              // Return early
              return 0;
          }
          num / 5
      }
      
      // Declare Car struct to describe vehicle with four named fields
      struct Car {
          color: String,
          transmission: Transmission,
          convertible: bool,
          mileage: u32,
      }
      
      #[derive(PartialEq, Debug)]
      // Declare enum for Car transmission type
      enum Transmission {
          // todo!("Fix enum definition so code compiles");
          Manual,
          SemiAuto,
          Automatic
      }
      
      fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
      
          // Use the values of the input arguments
          // All new cars always have zero mileage
          return Car{
              color: color,
              transmission: transmission,
              convertible: convertible,
              mileage: 0,
          };
      }
}