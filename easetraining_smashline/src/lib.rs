#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#[allow(unused_imports)]
#[allow(unused_macros)]

//mod training;
mod input;

#[skyline::main(name = "smashline_test")]
pub fn main() {
	//training::install();
	input::install();
}