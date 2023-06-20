/*
 * This file is known as a crate root for a binary because it matches 'src/main.rs'
 */

// Indicate that this code will use the file module (a module identified by the file name)
mod file;

// Indicate that this code will use the dir module (a module identified by its directory name)
mod dir;


// Indicate that this code will use an inline module (a module identified by its definition within this file)
// Note that 'inline' is available for use within this file. Changing this line to 'pub mod inline'
//  causes inline to be available outside this file.
mod inline {
    // The contents of a module are private to itself. In order for this submodule to be available
    //  in this file, main.rs, it must be declared public here.
    pub mod submod {
        pub fn hello() {
            println!("Hello from the inline submodule in main.rs");
        }
    }

    // Again, since the contents of a module are private, this function must be explicitly
    //  declared public to be available this file, main.rs
    pub fn hello() {
        println!("Hello from the inline mod in main.rs");
    }

}

fn main() {
    println!("*** This is the main function of the primary crate identified by src/main.rs ***");

    println!("\n===== Demonstrate module invocation in module search order. =====");
    inline::hello();
    file::hello();
    dir::hello();

    println!("\n===== Demonstrate SUBmodule invocation. =====");
    inline::submod::hello();
    file::submod::hello();
    dir::submod::hello();

    println!("\n===== Demonstrate Crate invocation. =====");
    inner_crate::hello();
    outer_crate::hello();
}
