#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod macro_input_version;
mod training;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    training::install();
    macro_input_version::install();
}