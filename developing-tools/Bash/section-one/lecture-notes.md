# Text Editors - In depth

Goal: you are familiar with how to use your text editor so that after this lecture you
can do basic file editing. 

## Vim

First, to edit a file, we need to create a file. You can use `touch file` to create one or use `vim file`
to create one. If you do not write anything to the file in Vim, the file will not be created. If you
use touch, the file will exist before editing. 

After running `vim file`, you enter the text user interface. Here, you can use commands to
manipulate text files. In the bottom left you can see nothing right now, this shows that we are in
command mode. If we want to enter insert mode, you can press `i` (sometimes I use `a` since my pinky
finger is on it already, but it is _slightly_ different than `i`). Here, you can type freely to edit
like any text editor. To return to command mode, you can hit escape. There is another mode called
visual mode which you can checkout
[here](https://opensource.com/article/19/2/getting-started-vim-visual-mode).

The power in Vim is going to come from the command mode, since insert mode is essentially the same
as notepad. Here are some helpful commands to know

- `$` will move your cursor to the end of the line, and `_` to the beginning
- `h, j, k, l` are analogous to `left, down, up, right` arrow keys
- `w` and `b` are for moving forward and backwards, one entire word at a time
- `/mode` will jump to the next occurence of the word "mode"
- `10gg` will move the cursor to line 10 
- `u` is for undo

Vim can chain commands together, pretty intuitively as well. For example, `d` is for delete, how
would we delete from the cursor to the end of the line, or the next word ? We would simply do `d$`
or `dw`. If we want to delete the entire line we are on, we can do `dd`. If we want to delete 20
lines from the cursor, we can do `d20`. Here you need to hit enter after to signal that you are not
doing `d20001` or something. 

To copy and paste, there are a few things we can do. When pasting in many lines of text, we can use
the command `:set paste`, the colon prefixes a longer command (read more
[here](http://infohost.nmt.edu/tcc/help/pubs/vi/web/colon-cmds.html])). This allows you to insert in
paste mode, which is depicted in the bottom left. To copy and paste in Vim, we can use the yank
(once again, you can chain this command as well). The command for this is `y` and you can paste
using `p`. 

One of the age old coding jokes is [How do I exit the Vim
editor](https://stackoverflow.com/questions/11828270/how-do-i-exit-the-vim-editor). It seems simple,
but people who are not used to using text only interfaces are used to hitting some sort of red "x".
A common thing to do is enter the command `:wq` which means write and quit. If you wrote some text
and did not write it, Vim will not let you leave. If you want to discard any changes you did write,
`:q!` is similar to force quit. 

## Emacs 

Here we will talk about how to do similar things in Emacs. Emacs is similar in that it has commands and edit ability, however the difference is that you do not go into a mode. Commands are prefixed with either the control key or the meta key, which is usually the alt/option key on your keyboard.

Like Vim, we can either do `touch` on a file first or just edit it with Emacs to create one doing `emacs file`.

After we have our file to edit opened, we can enter text just by typing away, no insert mode. The complexity in Emacs comes from the configurations, macros, and customizability one can do. But to edit files is simple. We can chain commands together as well which is just as useful in Vim as in Emacs, if not more useful. From here on out, let `C` be `control` and `M` be your meta key.

To move your cursor around, you can hold `C` and either `f, b, n, p` for forward a character, backwards a character, go to next line, and go to previous line. It is useful to use this not only because it keeps your fingers in the touch typing position, but you can chain commands together to move in more ways. For example `C-u 10 C-f` moves your cursor forward 10 characters.

Here are some more useful commands to know
- `C-a` and `C-e` move to the beginning and end of a line
- `M-f` and `M-b` move forward and backwards by one word
- `M-a` and `M-e` move forward and backward one sentence
- `C-v` and `M-v` scroll by one page (page being the number of lines visible in your terminal now)
- `M-<` and `M->` move to the beginning and end of a file
- `C-u` is a prefix that will take two arguments, a number and a command to repeat (shown above)
- `C-g` will stop a command that is taking too long to exit (similar to `C-c` when using Bash)
- `C-d` will delete the character after the cursor, the delete key will delete the one before
- `C-/` and `C-_` will both undo recent changes
- `C-k` and `M-k` will kill text from the cursor to the end of the line and the end of the sentence

The last command uses the term "kill". Here, that is similar to cut, and synonymous with yanking in Vim. So if `C-k` is to cut, then `C-y` is to paste. Notice the `y` which stands for yank.

Once we are done with editing the file, we are going to want to save and exit. `C-x C-s` will save the current buffer's progress to the file you are wanting to edit, buffer being the screen of edits you are currently looking at. To exit, do `C-x C-c`. 
