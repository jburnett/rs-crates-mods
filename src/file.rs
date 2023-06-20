/*
 * This file represents a file-based module, a module identified by its file name.
 */

// Publish this function to be available outside this file.
pub fn hello() {
    println!("Hello from the local file-based mod, file.rs");
}

// Publish this module to be available outside this file.
pub mod submod {
    // Publish this function in the module to be available outside this file.
    pub fn hello() {
        println!("Hello from the local file-based submodule in file.rs");
    }
}

