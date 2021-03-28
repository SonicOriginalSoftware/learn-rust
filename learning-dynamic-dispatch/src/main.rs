trait CanDoThings {
    fn do_it(&self);
}

/// A thing<br>
/// `struct` declaration define value members of the data type<br>
/// `struct` `impl` define function members of the data type
struct Thing {}

impl CanDoThings for Thing {
    fn do_it(&self) {
        println!("\tDid the thing! 🐳");
    }
}

struct Container<C: CanDoThings> {
    a_thing: C,
}

/// The program entry function
fn main() {
    let container = Container { a_thing: Thing {} };

    container.a_thing.do_it();
}
