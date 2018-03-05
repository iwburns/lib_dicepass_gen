#![feature(test)]

#![feature(plugin)]
#![plugin(phf_macros)]
extern crate phf;

extern crate rand;

mod defaults;

pub mod pass_gen;

pub use pass_gen::WordList;
pub use pass_gen::WordCount;
pub use pass_gen::PassGenConfig;
pub use pass_gen::generate;
