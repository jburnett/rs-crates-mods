/*
 * This file represents a directory-based module, a module identified by its directory name
 *  and containing a mod.rs file.
 */

// This module has a submodule which must be declared pub here to be available outside this file.
pub mod submod;

// Publish this function to be available outside this file.
pub fn hello() {
    println!("Hello from the local directory-based mod, dir/mod.rs");
}