// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat; // Máquina

    println!("{} is {} years old.", name, age);
}
