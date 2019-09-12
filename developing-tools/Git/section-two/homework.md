# Git as a team

Fill out the text file and submit it to Git like so
```
1. Link to the commit that solves the merge request
2. Link to the merge request on the web
```

### Merge Conflicts
What about what happens when things don't go as simple? In the case of branches, it is merge
conflicts. A merge conflict happens when a file you change in your branch is going to conflict
with another file. Here, conflict means the lines of a file in the master is going to overwrite the
lines of a file in the branch you want to merge in. 
All should have went well last time, but now, lets create a merge conflict.

- First, create a new git repository. make a new directory by doing `mkdir ~/git-test`. 
- Then `cd ~/git-test`, and do `git init .` to version control this directory.
- Then run `echo "test" > file`, and then `git add .` and `git commit -m "1"`. 
- Now switch to a new branch with `git checkout -b new`, and then overwrite the file by doing `echo
  "new" > file`, add it and commit your change.
- Then go back to master, run `echo "newer changes" >> file`, and then do a `git add` and commit
  your changes. 
- Now, while on master, run `git merge new`. We end up with an unexpected (in our case, expected),
  message. 

If you open `file`, you will see some `<<<<` and `======`. The `<<<` with `HEAD` at the end and one
with `new` at the end. The `======` is going to symbolize the two conflicting parts, one on each
side. Now, all you need to do is get rid of the arrows and equal signs, and make the text look like
how you want it to look. Then you can go about and do git add and commit changes. This is very
simple, but can be scarier when you are working on a larger code base or when you are afraid of
messing things up, but code is still only lines of text, and if things go bad you can always roll
back changes (more on that later). 

### Pull Requests
Now, do those same changes, except merge your code via a pull request and add a description to it
(it can be anything). Then try merging it via the web. 
