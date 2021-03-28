/// A thing<br>
/// `struct` declaration define value members of the data type<br>
/// `struct` `impl` define function members of the data type
struct Thing {}

trait DoesThings {
    fn do_it(&self);
}

impl DoesThings for Thing {
    fn do_it(&self) {
        println!("Did the thing! 🐳");
    }
}

/// The program entry function
fn main() {
    let thing = Thing {};

    thing.do_it();
}
