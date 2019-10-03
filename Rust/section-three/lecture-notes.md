# Pattern matching

There are more types of control flow operators than just `if/else` and `for`
loops. Say we have the enum from prelecture,
```
enum Year { 
    Freshman, 
    Sophomore,
    Junior,
    Senior
}
```

Say we want to print out the graduation date given the year of the student. How
do we do this elegantly? We can use the `match` operator. 

```Rust
fn get_approximate_graduation(student_year: Year) => usize {
    match student_year {
	Year::Freshman => 2020,
	Year::Sophomore => 2019,
	Year::Junior => 2018,
	Year::Senior => 2017
    }
}
```

We can also do more complicated things in match operators.
```Rust
fn get_approximate_graduation(student_year: Year) => usize {
    match student_year {
	Year::Freshman => 2020,
	Year::Sophomore => 2019,
	Year::Junior => 2018,
	Year::Senior => {
	    println!("Make sure you have enough credits to graduate!");
	    2017
	}
    }
}
```

You can grab values from enums. 
```Rust
enum Credits {
    u8
}

enum Year { 
    Freshman(Credits), 
    Sophomore(Credits),
    Junior(Credits),
    Senior(Credits)
}

fn check_credits(student_year: Year) {
    match student_year {
	Year::Freshman(cred) => {
	    println!("You have {} credits!", cred);
	}
	...
	Year::Senior(cred) => {
	    if cred < 84 {
		println!("You are not going to graduate on time.");
	    }
	    else {
		println!("You are on track to graduate.")
	    }
	}
    }
}
```
