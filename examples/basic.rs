extern crate lib_dicepass_gen;

use lib_dicepass_gen::*;

fn main() {
    let pg = PassphraseGenerator::new();
    println!("Your passphrase is: [{}]", pg.generate());
    println!("Your custom length passphrase is: [{}]", pg.generate_with_length(10));
}
