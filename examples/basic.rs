extern crate lib_dicepass_gen;

use lib_dicepass_gen::*;

fn main() {
    // the default word list
    let pg = PassphraseGenerator::new();
    println!("Your passphrase is: \n\t[{}]", pg.generate());
    println!("Your custom length passphrase is: \n\t[{}]", pg.generate_with_length(10));

    // the eff's long word list
    let pg = PassphraseGenerator::from_eff_long();
    println!("Your long-word passphrase is: \n\t[{}]", pg.generate());
    println!("Your custom length long-word passphrase is: \n\t[{}]", pg.generate_with_length(10));
}
