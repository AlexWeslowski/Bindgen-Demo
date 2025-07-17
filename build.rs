extern crate bindgen;
//extern crate cc; 
extern crate pkg_config;

use bindgen::callbacks::{ParseCallbacks, ItemInfo, ItemKind};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MyCargoCallbacks {
    rerun_on_header_files: bool,
    functions: Vec<String>,
}

impl MyCargoCallbacks {
    pub fn new(f: Vec<String>) -> Self {
        Self {
            rerun_on_header_files: true,
            functions: f,
        }
    }
    
    pub fn rerun_on_header_files(mut self, doit: bool) -> Self {
        self.rerun_on_header_files = doit;
        self
    }
}

impl Default for MyCargoCallbacks {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl ParseCallbacks for MyCargoCallbacks {
    fn header_file(&self, filename: &str) {
        if self.rerun_on_header_files {
            println!("cargo:rerun-if-changed={filename}");
        }
    }

    fn include_file(&self, filename: &str) {
        println!("cargo:rerun-if-changed={filename}");
    }

    fn read_env_var(&self, key: &str) {
        println!("cargo:rerun-if-env-changed={key}");
    }
    
    fn generated_link_name_override(&self, _item_info: ItemInfo<'_>,) -> Option<String> {
        match _item_info.kind {
            ItemKind::Function => {
                if self.functions.contains(&_item_info.name.to_string()) {
                    //println!("cargo::warning=ItemInfo.name: {}", _item_info.name);
                    return Some(_item_info.name.to_string());
                }
            },
            _ => {
            }
        }
        return None;
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let bln_static = false;
    let bln_dynamic = !bln_static;
    let lib_name = "addition";
    if bln_static {
        let _ = fs::copy(format!("/home/osboxes/Documents/Rust/bindgen-demo/src/{}.a", lib_name), format!("{}/{}.a", out_dir, lib_name));
        let _ = fs::copy(format!("/home/osboxes/Documents/Rust/bindgen-demo/src/lib{}.a", lib_name), format!("{}/lib{}.a", out_dir, lib_name));
        //println!("cargo:rustc-link-lib=static=stdc++");
        println!("cargo:rustc-link-lib=static={}", lib_name);
        println!("cargo:rerun-if-changed=src/{}.a", lib_name);
        println!("cargo:rerun-if-changed=src/lib{}.a", lib_name);
    }
    if bln_dynamic {
        let _ = fs::copy(format!("/home/osboxes/Documents/Rust/bindgen-demo/src/{}.so", lib_name), format!("{}/{}.so", out_dir, lib_name));
        let _ = fs::copy(format!("/home/osboxes/Documents/Rust/bindgen-demo/src/lib{}.so", lib_name), format!("{}/lib{}.so", out_dir, lib_name));
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
        println!("cargo:rerun-if-changed=src/{}.so", lib_name);
        println!("cargo:rerun-if-changed=src/lib{}.so", lib_name);
    }
    
    // /home/osboxes/Documents/Rust/mpmath
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-search=native={}", dir);
    println!("cargo:rustc-link-search=native={}/src", dir);
    
    if false {
        println!("cargo::warning=OUT_DIR: {}", out_dir);
        println!("cargo::warning=CARGO_MANIFEST_DIR: {}", dir);
    }
    
    let mut libraries = Vec::new();
    //libraries.push(pkg_config::probe_library("stdlib").expect("Failed to find stdlib library"));
    libraries.push(pkg_config::probe_library("mpfr").expect("Failed to find MPFR library"));
    libraries.push(pkg_config::probe_library("gmp").expect("Failed to find GMP library"));
    libraries.push(pkg_config::probe_library("gmpxx").expect("Failed to find GMPXX library"));
    for library in &libraries {
        for path in &library.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
        for lib in &library.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
    
    /*
    cc::Build::new()
        .cpp(true)
        .file("src/addition.cc")
        .include("/usr/include/")
        .include("/usr/include/x86_64-linux-gnu/")
        .include("/usr/lib/llvm-20/lib/clang/20/include/")
        .include("/usr/local/include/")
        .include("src/include/")
        .include("src/gmpfrxx/")        
        .cpp_link_stdlib("stdc++")
        .compiler("g++")
        .compile("addition");
    */
    
    // 
    // clang -std=c++11 -Wvla-cxx-extension -Isrc/include -Isrc/gmpfrxx/ -lmpfr -lgmp -lgmpxx src/main_sum/main_sum.cc -o src/obj/
    // 
    if true {
    let mut funcs: Vec<String> = Vec::new();
    funcs.push("add_long".to_string());
    funcs.push("add_double".to_string());
    let mycallbacks = MyCargoCallbacks::new(funcs);
    
    let bindings = bindgen::Builder::default()
        .header("./src/gmpfrxx/gmpfrxx.h")
        .header("./src/gmpfrxx/mpfr_mul_d.h")
        .header("./src/include/add.h")
        .clang_args(libraries[0].include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())),)
        .clang_args(libraries[1].include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())),)
        .clang_args(libraries[2].include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())),)
        //.clang_args(libraries[3].include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())),)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")        
        .clang_arg("-I/usr/include/")
        .clang_arg("-I/usr/include/x86_64-linux-gnu/")
        .clang_arg("-I/usr/lib/llvm-20/lib/clang/20/include/")
        .clang_arg("-I/usr/local/include/")
        .clang_arg("-I./src/")
        .clang_arg("-I./src/gmpfrxx/")
        .allowlist_function("add_long")
        .allowlist_function("add_double")
        .opaque_type("std::.*")
        .parse_callbacks(Box::new(mycallbacks))
        .generate()
        .expect("Unable to generate bindings");
    
    // /home/osboxes/Documents/Rust/bindgen-demo/target/debug/build/bindgen-demo_package-694d189ffb669d8d/out
    if true {
    let out_path = PathBuf::from(out_dir);
    //println!("cargo::warning=OUT_DIR: {}", out_path.display());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    }
    }
}