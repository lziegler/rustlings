// clippy3.rs
//
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        //my_option.unwrap(); // pointless -> will always panic
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: std::vec::Vec<u8> = vec![];
    //let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5); // resize returns (), NOT self
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    //value_a = value_b;
    //value_b = value_a;
    //std::mem::swap(&mut value_a, &mut value_b);
    (value_a, value_b) = (value_b, value_a); // destructuring assignment - relatively new (Rust 2021?)
    println!("value a: {}; value b: {}", value_a, value_b);
}
