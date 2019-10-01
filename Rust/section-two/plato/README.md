# Plato - A reduced version of the ls command

The first MP for this Rust section is going to be recreating a very reduced version of the `ls`
command. The functionality that will be required is 
- printing out the files and directories in the current directory
- printing out the permissions, owner, file size, time last modified, and the name of the file
- printing out the file size formatted to bytes, kbs, mbs, etc.
- accepting a directory or a file, similar to `ls my_file` and `ls my_directory` 
- printing out hidden files as well

This is similar to having some of the functionality of `ls -lha dir`, minus worrying about the `.` and
`..` directories. 

You will be responsible for filling in most of the functionality yourself! 

## Part 2
For this part you need to do a few things. You need to use
[structopts](https://docs.rs/structopt/0.2.18/structopt/) to parse command line
arguments easily. All the parameters are optional, but you should be able to see
whether or not the user entered `-l, -h, -a` (bools) and an optional file or directory (you should parse this as an `os_str` and have it be parsed into an `option<PathBuf>`. Fill out the `opts.rs` file with your struct. To test this, just print out the arguments provided in main. 
Furthermore, you need to fill out the two functions in the `list.rs` file. It
will be useful to read up on matching with the [result type](https://doc.rust-lang.org/std/result/).  

For testing, please use `cargo test-- --test-threads=1`. The default is to
parallelize tests, and that seems to cause some errors in the tests sometimes.
