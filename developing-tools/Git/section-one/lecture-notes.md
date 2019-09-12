# Learning how to use Git - The basics

This lecture will be on the basic, every day use of git that you will do with yourself. There are
more advanced ways of using git and ways of using git with a team. 

## Creating a git repo

First, we need to go to create a repository. 

- Go to `gitlab.engr.illinois.edu/YOUR-NETID`, and find where it says to create a new project. 
- Then you can add a project name, a description, and it is always best to initialize a new
  repository with a README`.
- Create a project.
- Now go to clone and clone with `https` hit the copy to clipboard button.
- Then in your terminal, and type `git clone https://gitlab.engr.illinois.edu/joshuad2/test.git`,
  and replace your `.git` file with your repo you copied to clipboard. 
- You will be prompted for your credentials to your gitlab account, after entering, you are done!

Now, with your new repo, let us add a file and change the README. 
- Go into your new repo and create a new file called `myfile` and write `this is my new file` in it.
- Also, go into your README and put the line `### This is a h3 header`. 
- Then we are going to use the command `git status`, you will notice that the `README.md` file is
  already tracked, but the newly created `myfile` is untracked, why ? 
- Use `git add` to move our new files and changed files to the staging area. Use `git status` to
  verify that something has happened.
- After this, we want to commit our changes, since we are ready to move files from the staging area
  to the local repository. 
- Now, we can use `git push` to move the changes in our local repository to the remote repository.

Now you have done the most basic of work flows in git! As a solo developer or on a team, most the
git commands you will use are these. We will learn what it means to work in git as a team, and then
also some more advanced usage of git later on (to be honest, anything outside clone, add/rm, commit,
and push can be considered intermediate or advanced git).
