#![feature(test)]
#![feature(extern_crate_item_prelude)]
#![feature(int_to_from_bytes)]
#![feature(slice_patterns)]
//just to dry up the code but not pay the price on benchmarks to pass a full String back (5000ns)
//function call wit u8 still adding 438ns to all of them.. sigh.. #[inline] doesnt help
#![feature(unsized_locals)]

mod flt;
mod string;

fn main() {}
