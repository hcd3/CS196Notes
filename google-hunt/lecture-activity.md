# Google Hunt Lecture
This lecture is going to be slightly different than previous lectures. Rather than having us walk around and answer your questions, you're going to be solving my problems today! Like you, I too have been struggling with Git, and I am going to need your help to solve the following issues that I have been having. For today's lecture, you'll be walking me through how to fix each of the issues that I have been having and providing me with an explanation as to why your solution will work. Enter your answers into the submission.txt file.

You guys are going to be on your own for this one. Your resources are Google and your fellow classmates. Good luck!
## Problem 1
I understand that a lot of you have been having some trouble pushing your answers to gitlab, and I have ran into a similar issue. When I try to push my code, I get the following error message.

  ![merge conflict](https://gitlab.engr.illinois.edu/cs196/private-content/google-hunt/raw/master/assets/mergeconflict.PNG).

  I read through the error message, and I run `git pull` as it suggests. Now, when I go to the file that I was working on, I see some weird characters `>>>` and `===` characters, like the following.

  ![conflict](https://gitlab.engr.illinois.edu/cs196/private-content/google-hunt/raw/master/assets/conflict.PNG).

  I am not really sure what to do at this point. I want to push my file to gitlab so that I don't get late credit on my assignment, but I don't want to go through the hastle of cloning the repo again. Please walk me through how to resolve this issue and how I can get my code pushed. I may run into this issue again, so if you could also tell me what I did to cause this error, that would also be great!

## Problem 2a
While working on some homework for one of my classes, I realize that I do not have python installed on my computer. So, I hop on the terminal and run `apt-get install python3.6`. When I try to run this command, I get a permission denied error. After doing a quick google search, I see that a possible solution is adding `sudo` to my command. I am a bit weary of just adding random terms to my command, so please explain to me what exactly `sudo` does and how it is going to fix my problem? While doing my own Googling, I read that `sudo` can also be a very dangerous tool, why is that?

### Problem 2.b
I have added `sudo` to my command, I am now trying to run `sudo apt-get install python3.6`. When I run this, I get a `unable to locate package` error. How can I fix this?

## Problem 3
So, I am back to working with Git now. I realized the errors of my ways from before, and consistently run `git pull` now to make sure I always have the latest versions of everything locally. My most recent time running `git pull`, I got the following error:

`error: The following untracked working tree files would be overwritten by merge:`

which was followed by a list of files. The files that were listed were files that I did not necessarily want on my git repo (they stored passwords and API keys). How can I resolve this issue, pull the code from the repository, and not have my password/key files overwritten?

## Problem 4
 `git pull` seems to really give me a lot of trouble. While I was running this command, my laptop ran out of battery and died. After charging it, and going back to run the command again. I got sevearl bizarre error messages. I then proceeded to run `git status` and got the following error:

 `error: unable to read tree object HEAD`

 I thought this was extremely strange, so I looked up the git documentation and tried running `git fsck` to see the validity of my tree object. This yielded `missing tree` and `dangling tree` messages. Please help me fix this issue: I just want to do my homework.
