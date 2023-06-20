# rs-crates-mods
Simple example of Rust's crate and module structures

While some languages and tooling allow code namespaces to differ from file directory structure, 
Rust takes the opinionated approach that project structure follows directory structure. This approach simplifies development efforts such as rapidly understanding a code base, debugging, etc.

This repo has several examples to demonstrate some of the rules in [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet) of the Rust Language Guide. The code in the crates and modules is about as simple as it can get, making it easier to understand the project structure and code path rules.

References interpreted relative to the crate root -- src/main.rs for binaries, src/lib.rs for libraries.  For example,

```
project-name
└--src
   └--main.rs
   └--foo.rs
```
where
- main.rs is the crate root
- foo.rs is a module file that main.rs might include using `mod foo;`