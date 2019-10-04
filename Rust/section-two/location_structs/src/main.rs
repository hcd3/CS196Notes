use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct TA {
    // Fill me out
    name: String,
    age: u32,
    location: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    course_staff: Vec<TA>,
}


fn average_age(staff_members: &Vec<TA>) {
    let mut amount = 0;
    let mut sum_age = 0;

    for x in 0..8 {
        sum_age += staff_members[x].age;
        amount = amount + 1;
    }

    println!("{}", (sum_age as f64) / amount as f64);
}

fn bay_area(staff_members: &Vec<TA>) {
    for x in 0..8 {
        if staff_members[x].location == "BAY_AREA" {
            println!("{}", staff_members[x].name);
        }
    }
}


fn main() {
    // Fill me out
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(args[1].to_string()).expect("Incorrect input!");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Incorrect!");
    let y:Data = serde_json::from_str(&contents).unwrap();

    if args[2] == "bay" {
        bay_area(&y.course_staff);
    }
    else if args[2] == "age" {
        average_age(&y.course_staff);
    }
}
