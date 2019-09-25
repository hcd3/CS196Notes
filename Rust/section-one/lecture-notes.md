# Ownership in Rust

A noticeable difference between Rust and other languages is ownership of variables. This differs
from other languages, it is a new approach to memory. In C, the programmer writes code explicitly
allocating and deallocating memory. If you want to create an array, you need to tell the operating
system that you need a certain amount of memory for that array. When done with this array, you have
to tell the operating system that you are no longer using it. In other languages, the allocating and
deallocating of memory is done for you via a garbage collector. Rust uses neither of these, so
working with memory is going to be a little different.



## The Stack and the Heap
Before we can get into understanding ownership, we need to know how memory works in a computer. All
data is built on top of the array data structure, in that, memory addresses are listed sequentially,
byte by byte. There are different ways you can use memory by specifying how you want to treat that
memory. 

A stack is just a general data structure about how we add and remove objects. You can imagine a
stack of plates. When you want to add one, you put it on the top. When you want to take one, you
take it from the top. Simply, the stack is a region of memory that is controlled using the Last In
First Out method. 

The heap is used when you have data that may change dynamically. Variables such as arrays or
integers have known sizes at compile time. Since the sizes are known, we do not have to worry about
reallocating memory, so it is easy to control. Some memory may grow over the course of a programs
lifetime, and is unknown at compile time how much it will grow. This is where we use the heap. It is
more complex, and thus slower than using the stack. This is not a bad thing, however, since you
cannot avoid writing code without using the heap. 


As stated in the beginning, most languages either manage memory themselves or the programmer has to
write code to manage memory, Rust does neither of these. Garbage collectors are slow and create
additional computational overhead, and managing memory yourself has been proven to be [very
hard](http://cs241.cs.illinois.edu/coursebook/Post_Mortems). Rust attempts to find the best
solution for the programmer, for speed, and for safety. 

### Managing Heap Memory
In CS241, you will learn the tradeoffs and implementations of how one does manage heap memory.
However, all you have to know for this is that there are some underlying algorithms that manage
allocating, reallocating, and deallocating heap memory. If you want to read more on this, you can
google all the different implementations and their trade offs. 

## Rules of Ownership
Now that we know a little bit about variables and data in general, specifically for Rust, we can
talk about ownership. Ownership is the method that Rust uses to keep track of memory. The rules of
ownership are
- Each value in Rust has a variable that is called its owner. Value here means the 5 in `let x = 5;`
- There can only be one owner at a time. 
- When the owner of the value goes out of scope, the value will be dropped. 

If a value can only have one owner at a time, that means that the value cannot be modified when
you do not expect it to be modified. This ensures data reliability. When the owner goes out of
scope, this means that the data will no longer be available to use when you are no longer using it.
These rules make sure that you cannot write code that falls into the same pitfalls as C/C++ does,
which are double frees, use after free, or memory corruption (not entirely important to know right
now, can google these if you are interested).

For example, the `String` class is stored on the heap. This is different than a string literal, in
that, the literal's size is known at compile time and can be stored on the stack reliably. A
`String` is used when you do not know the size of your data. An example could be when you are
reading input from a user and need to store that input. 

This is the example from the Rust book. 

```Rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
```

A more complex example would be this
```Rust
fn main() {
    let condition = true;
    if condition {
	let s = String::from("hello");
    }
    else {
	let s = String::from("world");
    }
    println!("{}", s);
}
```

The above code will cause a compile time error saying that it `cannot find value 's' in this scope`.
This is because the value has been dropped since the owner is out of scope. 

## Moving Data
In Rust, with non-heap data, we can do the following
```Rust
let x = 5;
let y = x;
```

This is because the size of the value is known, and thus can be copied easily. You can imagine the
operating system will allocate memory for the integer created by x, and then it will just allocate
memory as well for the integer created by y. Take this example from the Rust book.

```Rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

`s1` is not holding the literal value. It holds the location of the the data on the heap. This is
what we refer to as a pointer. A pointer is just another data type that just holds memory addresses.
`s1` for example, may be holding the value of `0x12345678`, which then at that memory address you
can find the data. In Rust, this object `s1` is a little more complex in that it holds the pointer,
bt also the length and the capacity. This way, you know the size of the data. You can get the value
of the `s1` by reading the memory at `0x12345678 + 5`, if 5 is the length of the string. 

Now, some languages copy memory differently, or you have to specify which type of copy you want. A
shallow copy simply would make s2 point to the same data in memory, `0x12345678`. If you change the
data in `s2`, then when you read from `s1` it also changes. The other type of copy is deep copy,
which is where the language will read the data at `s1` and allocate the same data that `s1` points
to at a new memory address. This is expensive since your data can be arbitrarily large. 

Rust does neither of these. Rust simply does not allow this to happen at all! The reason why is it
violates the rule that a value can only have one owner at a time. What Rust does instead is it
invalidates `s1`. Once you copy (Rust calls this move) the data, the previous instance of it is no
longer available to use. The code above will give the compiler error `use of moved value: `s1``. 

### Cloning
Rust does let you do a deep copy, but it makes you explicitly say you want to deep copy the data,
not hiding it. You need to call the `clone()` method on your code to do this.

### The Copy Trait
In the example above with setting the value of `y` using `x`, Rust is able to do this because the
integer type derives the `Copy` trait. This trait tells the compiler that you are able to know the
size of this data at compile time, making it such that you can use the stack and not the heap for
this data. Some types that implement the copy trait are `u32, bool, f64, char`. 

### Ownership with Functions
When calling a function with heap data as an argument it will move the data. When calling a function
with stack data as an argument, it will copy. The problem with passing heap data is that you no
longer will have the ability to use it, similar to the `s1` problem. 

You can get the ownership back if the function that took the data in as a parameter also returns it. 

```Rust
fn take_ownership_and_return(x: String) -> String {
    x
}

fn main() {
    let a = String::from("hello");
    let b = take_ownership_and_return(a);
}
```

Returning values and instantiation of new variables can get complex (imagine multiple values). It
may also not be the most readable.

## References
A lot of times, we need to pass data to functions not to change, but to use the values. Here we can
use references. References do not pass ownership since it shows that we are not going to be changing
the data. To pass an object as a reference to a function, we just need to add the `&` operator in
front of it. 

There may be times when we want to change the data in another function. To do this, we need to add
the `&mut` operator in front of our variables. We can only have 1 mutable reference and many
immutable references. The reason being that we have to follow the rules of ownership.

```Rust
fn main() {
    // immutable reference
    let a = String::from("hello");
    do_something_with_data(&a);

    // mutable reference
    let b = String::from("world");
    change_data(&mut b);
}
```

## Conclusion
By default, Rust makes you write really safe code. If you want to add some extra behavior with
moving and copying data, Rust makes you explicitly specify that you want to do this with your data.
The reasons for this being complex and difficult is that these errors in C/C++ are not known at
compile time, but only occur when the program is running. When the program is running, that means
the errors can be massive security vulnerabilities or performance hits. Rust ensures data
reliability by preventing data races, data corruptions, and other data vulnerabilities by default. 
