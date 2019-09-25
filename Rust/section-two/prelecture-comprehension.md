# Instructions
Please fill out the text file and submit it via Git like so
```
1. A
2. B
3. C
4. D
```

1. True or False : Can you have an empty field when you initialize a struct?
```
A. True
B. False
```

2. Which of the following pieces of code are valid using the following person struct?

```Rust
struct Person {
    first_name: String,
    last_name: String,
    address: String
}
```

I
```Rust
let father = Person {
    first_name = String::from("Bill"),
    last_name = String::from("Johnson"),
    address = String::from("1234 Street")
};

let mother = Person {
   first_name = String::from("Jill"),
   last_name = String::from("Johnson"),
   address = String::from("1234 Street")
};
```

II 
```Rust

let father = Person {
    first_name = String::from("Bill"),
    last_name = String::from("Johnson"),
    address = String::from("1234 Street")
};

let mother = Person {
   first_name = String::from("Jill"),
   last_name = father.last_name,
   address = father.address
};
```

III
```Rust
let father = Person {
    first_name = String::from("Bill"),
    last_name = String::from("Johnson"),
    address = String::from("1234 Street")
};

let mother = Person {
   first_name = String::from("Jill"),
   ..father
};
```

```
A. I, III
B. I, II
C. II, III
D. II, III, I
```

3. What is the correct way to call a struct's method?
```
A. struct.method()
B. struct.method(&self)
C. method(struct)
```

4. True or False : Every function in an `impl` block needs to take self as a
   parameter.
```
A. True
B. False
```
