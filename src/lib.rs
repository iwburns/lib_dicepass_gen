//!
//! A library for generating pass-phrases from dice-rolls against known word lists.
//!
//! ### Disclaimer
//! This project is currently a work-in-progress.  I make no guarantees about its correctness or
//! security.  I hope to provide such guarantees in the future.
//!
//! ### Usage
//! ```
//! use lib_dicepass_gen::*;
//!
//! fn main() {
//!     use lib_dicepass_gen::WordCount::*;
//!
//!     let config = PassGenConfig::from_eff_long(Default);
//!     let pass = generate(config);
//!
//!     println!("{}", pass);
//!     // should result in something like: "resubmit gallon faceted duplex octagon"
//!     # assert_eq!(pass.split_whitespace().count(), 5);
//! }
//! ```
//!

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
