use crate::chapter3::{fib, temperature};

mod hashmap_and_loops;
mod array_conditionals;
mod vars_and_print;
mod display_impl;
mod pattern_matching;
mod static_assignation;
mod handle_errors;
mod handle_errors_file_read;
mod memory_handling;
mod borrowing;
mod lifetimes;
mod lifetime_exercise;
mod traits;
mod traits2;
mod traits3;
mod traits4;
mod iterator;
mod generic_exercise;
mod iterator_exercise;
mod mod_test;
mod hash_test;
mod reg_ex;
mod modules;
mod guessing_game;
mod chapter3;
mod chapter_8;
mod chapter_3_good_way;
mod testing_1;

fn main() {
    /*     
        display_impl::main();
        vars_and_print::main();
        array_conditionals::main();
        hashmap_and_loops::main();
        static_assignation::main();
        pattern_matching::main();
        handle_errors::main();
        handle_errors_file_read::main();
        memory_handling::main();
        borrowing::main();
        lifetimes::main();
        lifetime_exercise::main();
        traits::main();
        traits2::main();
        traits3::main();
        traits4::main();
        iterator::main();
        generic_exercise::main();
        iterator_exercise::main();
        mod_test::main();
        hash_test::main();
        reg_ex::main();
        modules::main(); 
        guessing_game::main();
    */
    
    println!("{}", fib::fib(3));
    println!("{:.3}", temperature::to_farenheit(0.00));
    println!("{:.2}", temperature::to_farenheit(3.00));
    println!("{:.5}", temperature::to_farenheit(-3.00));
    println!("{:.5}", chapter_3_good_way::temperature::to_farenheit(-3.00));
    println!("{}", chapter_3_good_way::fib::fib(3));
}
