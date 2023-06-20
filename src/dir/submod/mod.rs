/*
 * This file represents a directory-based module, a module identified by its file name.
 * 
 * In this specific case, this module is a submodule of the 'dir' module due to the directory
 *  structure.
 */

pub fn hello() {
    println!("Hello from the local directory-based submodule in dir/submod/mod.rs");
}