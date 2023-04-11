// Different Rust types are
// integers
//    Length  Signed  Unsigned
//    8-bit   i8      u8
//    16-bit  i16     u16
//    32-bit  i32     u32
//    64-bit  i64     u64
//    arch    isize   usize

// floating-point numbers
//    Length  Signed
//    32-bit  f32
//    64-bit  f64

// Other Types
//    bool - true or false
//    char - Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
//    tuples - (Type, Type, Type)
//    arrays - [Type; Length]


fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // let mut number = 4.5;
    let number = 4.5;

    println!("The number is: {number}");

    number = 4.3;

    println!("The number is: {number}");
}
