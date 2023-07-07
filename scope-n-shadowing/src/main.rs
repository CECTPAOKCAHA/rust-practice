#![allow(unused)]
fn main() {

    let x = 1;
    {
        let x = 2;
        println!("{}", x);

    }
    println!("{}", x);    

    let xx = 1;
    let xx = "Hello";
    println!("{}", xx);   

    //Suffixes - specify the type of a numeric literal.

    let i = 42_f32;
    let y = 1_000_000;
    println!("{}, {}", i, y)
}
