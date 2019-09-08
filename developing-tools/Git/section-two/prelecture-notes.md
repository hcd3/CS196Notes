# Git 2

## Git as a team
Git is popular not because it can help you work better with yourself, but becaues it allows teams of
thousands of people to collaborate and work on projects together. By fully utilizing the power of
branches and git, working together on a project is easy and fast.

## Branches
We first learned about branches a bit before, but never really touched on it. You can think of the
git history as a tree. 

![](https://pedrorijo.com/assets/img/git-branches.png)

There are different branches in this picture, each spawning from different commits. A branch will
change some files, and then it will merge those changes into the master branch. It will not just add
in the last commit in your branch, but the entire history of commits in your branch. Now, not
everything is that easy and merging branches into the master branch (or any branch) may run inton
issues.

### How to use branches
To create a new branch, we use the command `git checkout -b my-branch`. Go to any of your git repos
for the class and use this. You will be told that you
switched to a new branch. You can also verify this by doing `git branch`, where you will see that
you have branched from master into your new branch. Now create a new file, commit your changes, and
push. 

Here you will notice that you get a new message that says you need to push your branch
upstream. Read more about it
[here](https://opensource.stackexchange.com/questions/993/what-does-upstream-mean), but like most
git messages, if you do what it says you will be fine.

Now, to merge this branch, go back to the master branch by doing `git checkout master`. Then, do
`git merge my-branch`. If all goes well, your commits from that branch will be merged into your
local repository, but you still need to push those changes to your remote repository. 

Now, you want to clean up when you are done as well. You can delete your local repository by running
the command `git branch -d my-branch` (make sure you are on master). To delete the branch in your
remote repository, go to the gitlab repo online and you can delete your branch there.

### Merge Conflicts
What about what happens when things don't go as simple? In the case of branches, it is merge
conflicts. A merge conflict happens when a file you change in your branch is going to conflict
with another file. Here, conflict means the lines of a file in the master is going to overwrite the
lines of a file in the branch you want to merge in. 
All should have went well last time we merged our branch, but we will find out what happens when it does not go well in lecture. 
