# lib_dicepass_gen

A library for generating pass-phrases from dice-rolls against known word lists.

### Disclaimer
This project is currently a work-in-progress.  I make no guarantees about its correctness or security.  I hope to
provide such guarantees in the future.

### Usage
```rust
extern crate lib_dicepass_gen;

use lib_dicepass_gen::PassphraseGenerator;

fn main() {
    let pg = PassphraseGenerator::new();
    println!("{}", pg.generate());
}
```
You should get something like: `ozone gorgeous dazzler aseptic cage dilated cytoplasm sugar`.

Custom length pass-phrases can be generated like so:
```rust
fn main() {
    let pg = PassphraseGenerator::new();
    println!("{}", pg.generate_with_length(10));
}
```

### About
`lib_dicepass_gen` uses one of the EFF's "short" word lists (which can be found
[here](https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt)) as its default word-list and simulates dice
rolls against that list to generate pass-phrases.

The words in this list provide 10.3 bits of entropy each, so you may want several words to create a strong enough
pass-phrase.  The default number of words (8) is considered to be "enough" entropy currently, but that will of course
change in the future.  More information about this list (and the EFF's other word lists) can be found
[here](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).

Future versions may allow custom word-lists to be used as well.
