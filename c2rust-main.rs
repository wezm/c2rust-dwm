#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(extern_types)]





extern crate libc;


#[path = "dwm.rs"]
pub mod dwm;
#[path = "util.rs"]
pub mod util;
#[path = "drw.rs"]
pub mod drw;

fn main() { dwm::main() }

