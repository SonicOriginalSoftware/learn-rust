mod a_module;
mod an_internal_library;
mod another_module;

use a_module::do_a_thing;
use an_internal_library::a_library::execute;
use another_module::another_module::do_another_thing;
use another_module::even_another_module::do_another_thing_even;

// mod declarations with blocks would be useful internally to the module
// but otherwise don't really benefit the structure for module importing
// which is a ginormous hell-hole

/// The program entry function
fn main() {
    do_a_thing();
    do_another_thing();
    do_another_thing_even();
    execute();
    println!("Hello, world!");
}
