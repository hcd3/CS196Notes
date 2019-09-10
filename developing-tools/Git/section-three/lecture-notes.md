# Undoing Changes

Now we learned a little bit about how to undo changes in git. Now let us figure out how to actually use these when necessary. 

For the purposes of this exercise, let us create a new repo and start from scratch. 
## Git revert

First we need to create a new file, and let us add a new file doing `touch file`. Then, we can add a line to it with `echo "line" > file`. Then `git add, commit, push` them. When you run `git log`, you should see 1 commit with your commit message. To revert this change, copy the commit hash and run `git revert <hash>`. Then you can run `git push`, and if you check the history on your repo, you will see a new commit prefixed with revert. 

## Git checkout --
Now, our repo should have nothing in it. Create a new file, and run `git add` and `git commit`. Then add a new line to that file, but do not run `git add`. After this, we can run `git checkout -- file` to undo any changes in our working directory to the last commit, which should be nothing. 

## git reset (--hard)
Now our `file` should have a single line in it. Let us add a new line to this file and commit those changes. If we want to roll back that commit, we can use `git reset --hard <hash>`. Run `git log` and find the last commit before the change you just made, where the file has only one line in it, copy the hash and run that command. 

## git reflog
Git reflog has almost all of your commits, even the deleted ones. To use this, we need to set up our history a bit. Add 3 commits by appending a new line in the `file`. Then, run `git log` and do `git reset --hard <hash>` on the commit from before the changes in this section. We can run `git checkout <hash> -- file` here to revert our changes back to the second commit. 

## git rebase -i
To start up this exercise, we need to set up some stuff. Create a file called `one` and commit it. Then do the same in the order of two, five, six, three, and four. Then once we are done with this, run `git rebase -i <hash>`, where `<hash>` is the commit where you created the file `one`. You will open up to an editor that tells you to pick certain commits. What you can do here is delete the lines of five and six, since they are out of order. Then you will end up without the files `five` and `six`. 