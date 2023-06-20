/*
 * This file is a crate root for a library because it matches 'src/lib.rs'
 * 
 * inner_crate is a crate identified by ./inner_crate/src/lib.rs relative to the primary crate
 *  root, src/main.rs
 */


// Publish this function to be available outside this file.
pub fn hello() {
    println!("Hello from the inner crate rooted in inner_crate/src/lib.rs which is a child of the primary src dir");
}