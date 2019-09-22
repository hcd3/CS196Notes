# Plato - A reduced version of the ls command

The first MP for this Rust section is going to be recreating a very reduced version of the `ls`
command. The functionality that will be required is 
- printing out the files and directories in the current directory
- printing out the permissions, owner, file size, time last modified, and the name of the file
- printing out the file size formatted to bytes, kbs, mbs, etc.
- accepting a directory or a file, similar to `ls my_file` and `ls my_directory` 
- printing out hidden files as well

This is similar to having the some of the functionality of `ls -lha dir`, minus worrying about the `.` and
`..` directories. 

You will be responsible for filling in most of the functionality yourself! 


## Part 1
Fill out the functions in the file `file.rs` in the `lib` directory. These are
helper functions for getting metadata from files. Some useful documentation for
this will be 
- [Rust metadata](https://doc.rust-lang.org/std/fs/struct.Metadata.html)
- [Rust unix metadata](https://doc.rust-lang.org/beta/std/os/unix/fs/trait.MetadataExt.html)
- [Rust pretty bytes](https://github.com/banyan/rust-pretty-bytes)
- [Rust users](https://docs.rs/users/0.9.1/users/struct.User.html)


### Submission
Push your changes to the `file.rs` file so that we can grade it. 
