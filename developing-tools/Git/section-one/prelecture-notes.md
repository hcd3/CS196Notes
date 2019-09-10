# Git 1

## What is Git ? 

Git is a "distributed version control system". But what does that mean? How is it different than a
regular file system? Why should I use it? The main purpose behind Git is to prevent you from losing
any history of your code, of your repository (a repository is just a folder). Once using a version
control system, it is hard to seriously destroy all your history and lose your entire code base, but
it can be easy to make small mistakes. The mystery behind this is because of what is going on in the
background.

## The four environments

In a non, version controlled, regular folder, there is only 1 environment, the working directory.
With git, there are four environments, three of these being on the local machine and the last one
being in some remote location. Before we can talk about these environments, we need to define some
terms.

### Definitions
- For git to be in a repo, you must first have a `.git` directory, so that git knows it can start looking at files in the directory. 
- An untracked file is something git does not know about. Git can see that it is in the working
  directory, but it is not something git cares about, it is not under the version control of git. A
  tracked file is the opposite.
- A branch can be thought of as one version of the code, that took a different path from the
  original branch.
- The original branch is called the master branch, which is the global source of truth. You can
  think of the master branch as the branch you would want your website running on. You can make a
  new branch for new changes, and then you can merge those changes into master when you want.
- To merge is to change the master branch in some way by editing, adding, or deleting parts. 

### Working directory
This environment is the same as what your operating system knows, and does not necessarily
distinguish any difference between things. These are all the files, tracked or untracked by git. 

### Staging Area
The staging area contains changes that you want git to know about, be it additional files, deleted
files, or edits to files. 

### Local Repository
The local repository contains the changes that you want to be made to your branch and realized in
there. These are the changes you would want mirrored in the remote repository

### Remote repository 
The remote repository is a mirror of your local repository, so that it contains the history of your
changes in your local repository. Having a remote repository means another place for your changes to
live and people can see it. 


How do your files move from environment to environment ? Well, theres a command for each one. To
move a file from your working directory to staging, use `git add myfile`. You can add as many files
and changes to files as you want. Similarly, you can say you want to delete a file in git by doing
`git rm otherfile`. 

Once you are satisfied with your changes, you can move them from the staging area to your local
repository with `git commit`. This command requires a message, however, which can be important to
reflect your changes. A good way to measure commits would be to use the [conventional commits
pattern](https://slides.com/marionebl/the-perks-of-committing-with-conventions#/0). Say we fixed a
bug in our bash script, our command may look like `"git commit -m "fix: Got rid of typo in script"`.
Also, you can specify any number of commits to make to your local repository. If you made two
changes, say you fixed a bug and added some comments to your code, you can space that out into two
commits.

Once you are satisfied with your commits to the local repository, you can send them to the remote
repository by doing `git push` .


### What next ? 


This is abstract, but we are going to go about this in class, right now, you just need to know some
of the logic of how git works in the background. 
