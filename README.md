## Comments Lister

My first **serious** Rust software.

It simply lists comments in files in given directory.

It also accepts different types of comments:

* C -> //
* Python -> #
* LISP -> ;

Multiline comments are not yet supported

```cargo run <directory> <comment_type>```

Example: &nbsp; ```cargo run src c``` -> This will look for C-style single line comments in src directory

Example results:

```
=========== RESULTS ===========

There are 2 comments in "main.rs"

// Test comment - at line 2
```