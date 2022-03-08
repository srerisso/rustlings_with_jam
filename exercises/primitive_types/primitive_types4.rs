// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// https://doc.rust-lang.org/rust-by-example/primitives/array.html

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4]; // no incluye los indexexs supongo. Sin lógica.

    assert_eq!([2, 3, 4], nice_slice)
}
