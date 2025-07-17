#![allow(warnings)]

use gmp_mpfr_sys::gmp::{self};
use gmp_mpfr_sys::mpfr::{self};
use std::mem::MaybeUninit;

mod ffi {
    use gmp_mpfr_sys::gmp::{self};
    use gmp_mpfr_sys::mpfr::{self};
    
    // include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    
    /*
    unsafe extern "C" {
        #[link_name = "\u{1}add_long"]
        pub fn add_long(i: ::std::os::raw::c_long, j: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
    }
    */
    
    #[link(name="addition")]
    unsafe extern "C" {
        pub fn add_long(i: ::std::os::raw::c_long, j: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
        //pub unsafe fn add_double(i: ::std::os::raw::c_double, j: ::std::os::raw::c_double) -> ::std::os::raw::c_double;
    }
    
}

pub fn add_i64(i: i64, j: i64) -> i64 {
    unsafe {
        return ffi::add_long(i, j);
    }
}

pub fn add_f64(i: f64, j: f64) -> f64 {
    return i + j;
}



