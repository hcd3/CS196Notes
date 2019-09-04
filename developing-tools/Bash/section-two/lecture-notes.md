# Intermediate Bash
Goal: Bash is not something that we theorize about, it is something that is used most like a tool, such as a [hammer](https://en.wikipedia.org/wiki/Heideggerian_terminology#Ready-to-hand).
## Redirection
Redirection is when we take the output of one command and instead of just displaying that output (to STDOUT), we want to direct that output to a file. For example, the command `echo "Hello" > myfile` will not display anything into the terminal. If you check the file `myfile`, you will see "Hello" in that file. We can do add more text to the file by first doing `cp myfile oldfile` and then `cat oldfile >> myfile` (the `cp` is so we do not have an infinite loop doing `cat myfile >> myfile`) . Two arrow operators means that we are going to append the output of the command to the end of our file `myfile`. We can redirect the other way by doing `cat < myfile`, and you can see the outuput for yourself.

## Piping
Along with redirecting input from STDOUT to files, we can move input between commands as well. First, lets create a few more files with `touch a b c d e f`. If we do `ls`, we should see `a b c d e f oldfile myfile`. Let's say we want to get the first 3 files in the directory, we can do `ls | head -3`. We can also do `ls | head -3 | tail -2 | head -1` , chaining as many as we like. If we were not using pipes, we could do `ls > output` and then `head -3 output` to get the first 3. However, we will not always want to write to a file, we may just need to use the output immediately.

## Logical Operators
Like many languages, Bash has logical operators. Bash will check if a command worked or not by cecking the exit status of each statement linked by logical operators. Lets take a look at this command.
```
touch myfile
[[ -e "myfile" ]] && echo "myfile exists"
rm myfile
[[ -e "myfile" ]] && echo "myfile exists"
```

First, we created a file, and then we ran a command that checks to see if a file exists. If it does, then we print a message saying it does exist. After we remove the file, the command will not do anything, since the first condition will cause the program to exit since the first half evaluated to false, and false && (true or false) will always evaluate to false. Another way to look at this is that we use the `&&` operator when we want the commands to work in order. An example of this would be `make && ./my-executable`. This would build an executable in C and then run that executable only if `make` succeeded.

Another thing we can do is `||`, whick will run the second command if the first fails.
```
touch myfile
[[ -e "myfile" ]] || echo "myfile does not exist"
rm myfile
[[ -e "myfile" ]] || echo "myfile does not exist"
```
This will do nothing with the first conditional, but after removing `myfile`, it will show us that it does not exist.


We can also use `;` to chain commands, which will run commands regardless of if the first succeeded, essentially allowing you to combine two commands into one line.

# Scripting

Note : To run these, create a file called `script.sh`, put in the code, and use `bash script.sh`.

### Variables
```
x=21
echo $x
echo "I am $x years old"
echo "I am ${x} years old"
```

### Conditionals
```
# Fizz Buzz
x=10
if [[ 0 -eq "(x%3)+(x%5)" ]]; then
    echo "Fizz Buzz"
elif [[ 0 -eq "(x%3)" ]]; then
    echo "Fizz"
elif [[ 0 -eq "(x%5)" ]]; then
    echo "Buzz"
fi

```

### Loops

```
for i in {1..10}; do
    echo "Hello $i"
done
```

### Functions

```
greetPerson() {
    echo "hello $1"
}

greetPerson "Josh"
```
