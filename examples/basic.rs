extern crate lib_dicepass_gen;

use lib_dicepass_gen::*;
use lib_dicepass_gen::WordCount::*;

fn main() {
    // the eff's short list (with unique prefixes)
    let mut config = PassGenConfig::from_eff_short_2(Default);
    println!("Your passphrase is: \n\t[{}]\n", generate(config));

    config.word_count = Custom(10);
    println!(
        "Your custom length passphrase is: \n\t[{}]\n",
        generate(config)
    );

    // the eff's long word list
    let mut config = PassGenConfig::from_eff_long(Default);
    println!("Your long-word passphrase is: \n\t[{}]\n", generate(config));

    config.word_count = Custom(10);
    println!(
        "Your custom length long-word passphrase is: \n\t[{}]\n",
        generate(config)
    );
}
