#![allow(unused)]
use std::io;
fn main() {

    //COMPOUND TYPES - Tuples & Arrays
    //Max value of tuple is 12
    let student_a = ("Heath", 'A', 3.58);
    //let name_student_a = student_a.0;
    //let grade_student_a = student_a.1;
    //let gpa_student_a = student_a.2;

    let (name_student_a, grade_student_a, gpa_student_a) = student_a;

    println!("My name is {}, my class grade is {}, my GPA is {}.", name_student_a, grade_student_a, gpa_student_a);

    //Arrays - [] - can store up to 32 items and they have to be similar data types.
    let students = ["Heath", "Bob", "Linda"];
    println!("The first student in our array is {}", students[0]);

    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[1..3];
    println!("{:?}", slice);
    slice[0] = 6;
    slice[1] = 7;

    print!("{:?}", arr);
}
