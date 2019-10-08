# Plato - A reduced version of the ls command

The first MP for this Rust section is going to be recreating a very reduced version of the `ls`
command. The functionality that will be required is 
- printing out the files and directories in the current directory
- printing out the owner, file size, time last modified, and the name of the file
- printing out the file size formatted to bytes, kbs, mbs, etc.
- accepting a directory or a file, similar to `ls my_file` and `ls my_directory` 

You will be responsible for filling in most of the functionality yourself! 

## Part 3
Using the file data we have gathered so far, we need to put the data into a
struct called `ListItem`. There are two functions to fill out. `get_file_data`
will get the information needed about a certain file given the current directory
and the args provided in the command. `create_list_items` will return an array
lof `ListItem` structs about some files and directories.



Note : You need to get the relative path and not the absolute path of files. The `pathdiff` crate might come in handy. 
