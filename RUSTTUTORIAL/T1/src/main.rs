use std::collections::HashMap;

mod b {

    pub mod c;
}
mod file2;

use std::io; // for inputs std = crate io = module
fn main() {
    println!("Hello World");
    let mut x = 12;
    // mutable variable;
    println!("{}", x);
    x = 187;
    println!("{}", x);
    let x = "Hello !"; // redefining
    println!("{}", x);

    let y: i32 = -2; // normal 32bit signed int, i8,i16,i32,i64,i128
    let z: u32 = 2; //unsigned int, same principle
    let f: f32 = 2.2; // also f64
    let b: bool = false;
    let c: char = 'd';
    let tup: (i32, bool, char) = (1, true, 's');
    println!("{}, {}, {}, {}, {}, {}", tup.0, y, z, f, b, c);
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; //all elements have to be the same element
    arr[2] = 69;
    println!("{}", arr[2]);

    let mut inp = String::new(); // :: to go from module to function or crate to module
    io::stdin()
        .read_line(&mut inp)
        .expect("failed to read line"); // gives ownership over to outside scope, so that the value is not deleted after the function has run, that's why we use & and mut to make the reference mutable
    println!("{}", inp);

    // different datatypes e.g. u8 and i8 can not be directly added, they need to be casted
    // if we use arithmetic operations, the result will be of the same type as the operands i8 / i8 = i8
    let n1: f64 = 52.5;
    let n2: f64 = 2.5;
    let n3: i16 = n1 as i16 / n2 as i16;
    println!("{}", n3);

    let mut inp2 = String::new();
    io::stdin()
        .read_line(&mut inp2)
        .expect("failed to read line");
    let int_input: f32 = inp2.trim().parse().unwrap(); // convert from str to f32
    println!("{}", int_input + 16.0);

    if int_input > 10.0 {
        println!("ZU GRO?E ZAHL!");
    } else {
        println!("Stabile Zahl bro!");
    }
    println!("{}", test(14, 12));
    let newnum: i32 = {
        let g = 13;
        g + 19
    }; // like an anonymous function
    println!("{}", newnum);
    is_palindrome(123);
    /*
    Stack    vs     Heap
    LIFO Stack -> very fast
    when defining a variable, it is stored in the stack
    when the variable goes out of scope, it is removed
    everything on the stack has a fixed size
    objects with dynamic size are stored in the heap
    we are allocating space on the heap instead of just putting it onto the stack -> slow
    -> instead of copying the values, the stack stores a pointer to the object in the heap
    the heap is like a big map that maps values to an address
    both are in RAM
    */

    /* Different string types:
    Two string encodings: UTF-8 and ASCII
    ASCII -> 1 char = 1 byte -> only 256 different characters
    UTF-8 -> 1 char = 1-4 bytes -> way more possibilities while still being efficient

    Strings store str length as metadata, no null terminator, which makes operations like .length very fast
    are immutable by default

    String
        heap allocated, owns underlying memory | has pointer, length and capacity
        useful for creating and modifying strings dynamically at runtime
        wrapper of Vec<u8>
    &str: string slices
        useful for reading only, without modifying the string
        allows to view a part of the data without referencing the entire collection
        immutable sequence of UTF-8 bytes of dynamic length somewhere in memory

        other str pointer types:
        Box<str> -> heap allocated, owns underlying memory -> basically like String bit without capacity therefore changing content is possible but changing size is not
        Arc and Rc<str> -> reference counted pointers, can be shared between multiple owners, use when working with big strings that are expensive to clone

    */
    let normal_str: String = String::from("Hello World");
    let normal_str_slice: &str = &normal_str; // doesn't own the data, just a pointer to the data
    let literal_str: &str = "Literal"; // references to the data section of compiled binary
    let raw_str = r#"Moino""""""GG'W'DWD'W"#;

    /*
    Ownership model:
    Each value in Rust has a variable that’s called its owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.
    Simple types stored on the stack are copied by default

     */

    let s1: String = String::from("Hello World");
    let s2: String = s1; // moves the ownership of s1 to s2, s1 is no longer valid, you can also clone
    let g = is_palindrome(1344); // g now takes ownership of the return value of is_palindrome
    let lmao: i32 = 245;
    let lmao2: i32 = takes_ownership_and_gives_back(lmao);

    let v1: String = String::from("12345678");
    let len: usize = dont_take_ownership(&v1);
    println!("{}, {}", v1, len);

    let mut v11: String = String::from("12345678");
    dont_take_ownership_but_change(&mut v11);
    println!("X after changing:{}", v11);

    let slice1 = &s2[0..3]; // 3 is exclusive

    let mut counter: i32 = 0;
    let stop = loop {
        if counter == 5 {
            break counter;
        }
        println!("{}", counter);
        counter += 1;
    }; // just goes until it hits the break statement
    let test123: [i32; 5] = [1, 2, 3, 4, 5];

    for element in test123.iter() {
        println!("{}", element);
    }

    for i in 0..test123.len() {
        println!("index: {}, element: {}", i, test123[i]);
    }
    println!("Stopped at {}", stop);
    println!(
        "{}",
        longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ])
    );

    file2::abcde();
    b::eafgh();
    b::c::lmasdsasd();
}

fn test(x: i32, y: i32) -> i32 {
    println!("Hello World");
    return x * y; // return not necessary
}
pub fn is_palindrome(x: i32) -> bool {
    let mut x_str: String = x.to_string();
    x_str = x_str.chars().rev().collect();
    println!("{}", x_str);
    true
}
fn takes_ownership_and_gives_back(x: i32) -> i32 {
    println!("Ownership inside of function");
    return x;
}
fn dont_take_ownership(x: &String) -> usize {
    // This borrows the value of x, but doesn't take ownership of it
    println!("{}", x);
    let length = x.len();
    length
}

fn dont_take_ownership_but_change(x: &mut String) {
    // This borrows the value of x, but doesn't take ownership of it

    println!("X before changing:{}", x);
    x.push_str(" changed");
}
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut subs: Vec<&str> = Vec::new();
    for word in strs.iter() {
        for i in 0..word.len() {
            subs.push(&word[0..i + 1]);
        }
    }
    println!("{:?}", subs);
    let mut hashmap: HashMap<&&str, u16> = HashMap::new();
    for substr in subs.iter() {
        if hashmap.contains_key(substr) {
            hashmap.insert(substr, hashmap[substr] + 1);
        } else {
            hashmap.insert(substr, 0);
        }
    }
    let mut cmax: u16 = 0;
    let mut cmaxStr = String::new();
    for (key, val) in hashmap.iter() {
        if val.clone() > cmax {
            cmax = val.clone();
            if key.len() > cmaxStr.len() {
                println!("Found new max: {}", key);
                cmaxStr = key.to_string();
            }
        }
    }
    println!("{:?}", hashmap);
    println!("{:?}", cmaxStr);
    cmaxStr
}
