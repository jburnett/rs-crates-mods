/*
 * This file is a crate root for a library because it matches 'src/lib.rs'
 * 
 * outer_crate is a crate identified by ../outer_crate/src/lib.rs relative to the primary crate
 *  root, src/main.rs
 */

// Publish this function to be available outside this file.
pub fn hello() {
    println!("Hello from the OUTER crate rooted in src/lib.rs in outer_crate, a sybling of the primary src dir");
}