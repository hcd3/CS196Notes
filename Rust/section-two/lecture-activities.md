# Working with Structs

For these lecture activities we are going to be working with structs. To make it
more interesting, we are going to use the `serde` crate to work with json and
read the json into structs in Rust. 

The program will take the json file as input and also the string `bay` or `age`
to determine which function to run. Fill out the `location_structs` main file. 

The program takes an argument of `bay`or `age`. 

## Get all the course staff from the bay area
Fill out the function that takes an input of the course staff roster and prints out
all of the names of the staff that is from the bay area. 

## Average age
Fill out the function that gets the average age of all the course staff members. 

## Example function
```Rust
josh$ cargo run data.json bay
rishi
zac
peter
melissa
jatin
clara
```

```Rust 
josh$ cargo run data.json age
19.4
```


