# Instructions
Please fill out the text file and submit it via Git like so
```
1. D
2. B
3. C
4. A
```

# Questions

1. Fill in the blank. {} is always immutable. {} is immutable by default. {} is mutable.
```
A. let mut x, let x, const X
B. let x, let mut x, const X
C. const x, let mut x, let x
D. const X, let x, let mut x
```
2. Look at the following piece of code. What is not a way to solve it? 
```Rust
fn main() {
    let x = 0;
    x = x.to_string();
}
```
```
A. make the second line `let x = x.to_string()`
B. combine the lines into one line `let x = 0.to_string()`
C. make x mutable by doing `let mut x = 0`
```
3. True or False : Can you have an array with different types? 
```
A. True
B. False
```
4. Look at the following piece of code. What is wrong with it?
```Rust
fn main() {
    let condition = true;
    let x = if condition {
        5
    } else {
        "a"
    };

    println!("The value of x is: {}", x);
}
```
```
A. there are missing semi-colons in the conditionals
B. there are no return statements in the code
C. the values are of different types, causing a compiler error
```
