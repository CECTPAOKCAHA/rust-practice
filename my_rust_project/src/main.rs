fn main() {
    //Scalar types: int, float, boolean, char
    //Unsigned - never negative - u8, u16, u32, u64, u128, usize
    //signed - can be negative and positive - i8, i16, i32, i64, i128, isize
    println!("Max size of a u32: {}", u32::MAX);
    println!("Max size of a u64: {}", u64::MAX);
    println!("Max size of a u32: {}", i32::MAX);
    println!("Max size of a u64: {}", i64::MAX);

    //int literals - decimals 1000, binary 11110000

    //floats - f32, f64 - 3.14
    println!("Max size of a f32: {}", f32::MAX);
    println!("Max size of a f64: {}", f64::MAX);

    //boolean - true or false

    //character - char - 4 bytes

    println!("{}", 'A');

    // Variables
    let mut hello = "Hello, world!";
    println!("{}", hello);
    hello = "Hello, again!";
    println!("{}", hello);

    let x = 5;
    let y = 6;
    println!("Math in Rust: {} + {} = {}", x, y, x + y);

    //Constants - can be placed outside of functions - global immutable variables. Very fast.
    const NUMBER: i32 = 17;

    println!("{}", NUMBER);

}
