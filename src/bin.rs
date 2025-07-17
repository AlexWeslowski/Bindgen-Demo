#![allow(unused_imports)]
extern crate bindgen_demo;
use bindgen_demo::{add_i64, add_f64};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;


// 
// target/debug/addition_exe
// target/release/addition_exe
// 
pub fn main() -> std::io::Result<()> 
{
    unsafe { env::set_var("RUST_BACKTRACE", "1"); }
    
    let dir = env::var("CARGO_MANIFEST_DIR");
    if dir.is_ok() { println!("CARGO_MANIFEST_DIR = {}", dir.unwrap()) };
    let out_dir = env::var("OUT_DIR");
    if out_dir.is_ok() { println!("OUT_DIR = {}", out_dir.unwrap()) };
    
    if true {
        let dir: PathBuf = env::current_dir()?;
        println!("The current directory is: {}", dir.display());
        if false {
            let paths = fs::read_dir(".")?;
            for path in paths {
                println!("{:?}", path?.path());
            }
        }
    }
    
    if true {
        let i = 3;
        let j = 5;
        let k = add_i64(i, j);
        println!("{} + {} = {}", i, j, k);
        
        let fa = 3.14159;
        let fb = 2.71828;
        let fc = add_f64(fa, fb);
        println!("{} + {} = {}", fa, fb, fc);
    }
    
    Ok(())
}

