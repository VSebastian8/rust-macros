#![crate_type = "lib"]

#[test]
#[unsafe(no_mangle)]
#[allow(dead_code, unused_variables)]
#[rustfmt::skip]
fn my_func() {
      let   x = 2 ;
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Bit {
    Zero,
    One,
}

#[derive(Debug, Default, Clone, Copy)]
struct Wrapper {
    val: i32,
}

fn main() {
    println!("Comprehension Macro!");

    // Derive Macros
    let bit = Bit::Zero;
    println!("Bit: {:?}", bit); // Bit: Zero
    assert_eq!(bit, Bit::Zero);
    assert!(Bit::Zero < Bit::One);

    let wrap = Wrapper::default();
    let wrap2 = wrap.clone();
    assert_eq!(wrap.val, wrap2.val);
    println!("{:?}", wrap2); // Wrapper { val: 0 }
}
