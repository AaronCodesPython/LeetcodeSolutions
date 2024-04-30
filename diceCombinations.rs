use std::io; // for inputs std = crate io = module

fn main() {
    println!("Hello World");
    let mut x = 12;
    // mutable variable;
    println!("{}", x);
    let x = "Hello !"; // redefining
    println!("{}", x);

    let y: i32 = -2; // normal 32bit signed int, i8,i16,i32,i64,i128
    let z: u32 = 2; //unsigned int, same principle
    let f: f32 = 2.2; // also f64
    let b: bool = false;
    let c: char = 'd';
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}", tup.0);
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; //all elements have to be the same element
    arr[2] = 69;
    println!("{}", arr[2]);

    let mut inp = String::new(); // :: to go from module to function or crate to module
    io::stdin()
        .read_line(&mut inp)
        .expect("failed to read line"); // gives ownership over to outside scope, so that the value is not deleted after the function has run, that's why we use & and mut to make the reference mutable
    println!("{}", inp);

    // different datatypes e.g. u8 and i8 can not be directly added, they need to be casted
}
