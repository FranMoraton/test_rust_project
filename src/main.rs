pub mod hashemap_and_loops;
pub mod array_conditionals;
pub mod vars_and_print;
pub mod handle_errors;
pub mod display_impl;

fn main() {
    display_impl::main();
    vars_and_print::main();
    array_conditionals::main();
    hashemap_and_loops::main();
    handle_errors::main();
}