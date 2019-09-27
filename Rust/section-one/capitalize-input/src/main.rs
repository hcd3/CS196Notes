use std::io;
use std::io::prelude::*;

// Echoes the input like so
// You entered hello world
// DOES NOT RETURN ANYTHING
fn echo_input() {
    // Fill me out
    let mut s = String::from("hello");
    println!("You entered {}", s);
}

// Changes the input data to be uppercase
// Look at std::string::String for help
// DOES NOT RETURN ANYTHING
fn uppercase() {
    // Fill me out
    println!("{}", s.to_uppercase());
}

// Reads standard input and uppercases the input
fn main() {
    echo_input();
    uppercase();
}

