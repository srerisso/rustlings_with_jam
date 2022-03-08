// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` for hints!

// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
