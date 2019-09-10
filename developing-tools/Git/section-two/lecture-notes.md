# Git as a team

Learning how to use git as a team is the most important part of git.

## Branches

Generally as a team, you should make a branch for a new change to the master branch you want to introduce. At a larger company, you usually have a project you are working on with a certain amount of tasks that make up that projects entirety. For example, you may be making an in app product announcement page, and you break it up into 2 parts of front end and back end and 10 tasks each, for a total of 20 branches. Another case is that an error surfaces in the production code and you need to fix it, warranting a new branch to fix that bug. 

### Naming branches

There are many ways to name branches, but it is usually up to your team. It is usually best to tie it to the task number that it is fixing. 

Say your company uses Jira (a task management system), and your team is called Infra. Say you need to update the current version of Rust for the repo because of a security flaw. Your branch may be named `joshdunigan/Infra-214/update-rust`, where Infra-214 is the task name related to what it is in Jira. 

## Commit messages

Commit messages are a way for people to see what changes you made at a glance. One thing that engineers or professors might tell you is that you should "commit often" and that is the end of the advice. They may even say "write meaningful commit messages", but most the time that does not mean anything and when learning it is hard to understand what it should be. 

A better way to think about it is that you should use messages as a way to break your changes into meaningful and disjoint sets of changes. 

### Example of conventional commit message usages

Say you are working on a very large codebase, and a bug is reported. You as an experience engineer know that you need to go in and fix the bug, but that is not all that you see that needs to be fixed. This code was last touched 2 years ago. The engineer before did not write any documentation or tests for this as well. Not only is the logic for the code faulty, the code is not very pretty looking. 

Here, a bad engineer would just do it all in one commit. A regular engineer may go in and make one commit for each file, the test file and the actual code. An experienced engineer may even have good commit messages that show what they did. But a great engineer does more. A great engineer is just a good engineer with great habits. 

A great habit to use for commit messages is to use the [conventional commits ](https://slides.com/marionebl/the-perks-of-committing-with-conventions#/1) standard. What this does is makes you break your commits into disjoint changes, such as tests, bug fixes, documentation, etc. The way it looks is like `feat(my-file): added get user address route`. Feat corresponds to the type of change, the `my-file` corresponds to the scope of the changes, and the last is the message explaining what you did. 

Back to our example, a great engineer would realize that there are 4 disjoint changes to make to this code base, and plan their commits accordingly. The 4 changes would be `fix`, `test`, `docs`, and `refactor`. Then these would be the changes necessary to fix the bug. 

### Why does this matter ? 
Not only to help make you a better programmer, and to help your team view your code changes easier, it is safer to use better commit messages. The more disjoint your commit messages are, the easier it is to roll back changes to a specific code change, or to decouple good changes with changes that introduced an error. When looking for that error, it will be easier to know where to look when it was introduced. Since tests and documentation are not usually the causes of errors, you would just go and look at the other two commits of `fix` and `refactor`. 

## Pull Requests 

Now, once you made your branch, you made your commits, you need to make a pull request. In pre-lecture, we merged code just via the command line, however, real life is not like that, for compliance reasons and for engineering reasons. For compliance reasons, you do not want someone putting in malicious code into the code base. For engineering reasons, you want to make sure that no one is putting in poorly written code into the code base. 

The difference between merging in the command line and making a pull request is that you can be more descriptive about the changes and allow for a conversation for feedback to happen about the changes before it is approved by someone to merge in. 


### Descriptive
A lot of things can go into the description of a PR. For example, some may be
- A picture of the changes you made (if they are front end facing)
- A link to the task in Jira or the issue on git(github or gitlab)
- A description of what was wrong with the code before
- A description of what changes you made to fix the code
- How to test your changes (log in as a user, go to their settings, and then ...)

