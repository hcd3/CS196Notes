# Instructions
Please fill out the text file and submit it via Git like so

```
1. A
2. B
3. C
4. D
5. A
```

# Questions
1. What are reasons that sudo will not work correctly ?
```
A. The user is not in the sudoers file
B. The password was incorrect
C. The user does not have sudo access to that command 
D. A and B are correct
E. B and C are correct
F. All the above are correct.
```
2. Which command searches a folder and _all_ of its contents for the word `josh` and ignores casing.
```
A. `grep /my-folder "josh"`
B. `grep "josh" -r /my-folder`
C. `grep -ir "josh" /my-folder`
```
3. What is the order of commands to correctly create this Tmux setup ?

![](../assets/Screen_Shot_2019-06-08_at_7.09.12_PM.png)
```
A. `ctrl+b` % -> `ctrl+b "`
B. `ctrl+b` " -> `ctrl+b %`
```
4. What does `ctrl+b o` do? 
```
A. Renames the current pane
B. Kills the current window
C. Shows the time
D. Selects the next pane in the current window
```
5. In the following function in someones `.bashrc` , what does `$1` represent?
```
function gall() {
    git add -u
    git commit -a -m "$1"
    git push
}
```
```
A. The first environment variable 
B. The argument to the function
C. The string "$1" 
D. A variable defined before as `$1`
```

