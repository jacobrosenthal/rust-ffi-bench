#![feature(uniform_paths)]
#![feature(test)]
#![feature(slice_patterns)]
//just to dry up the code but not pay the price on benchmarks to pass a full String back (5000ns)
//function call wit u8 still adding 438ns to all of them.. sigh.. #[inline] doesnt help
#![feature(unsized_locals)]

extern crate test;

mod flt;
mod string;

fn main() {}
