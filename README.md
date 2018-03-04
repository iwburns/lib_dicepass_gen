# lib_dicepass_gen

A library for generating pass-phrases from dice-rolls against known word-lists.

### Usage
```rust
extern crate lib_dicepass_gen;

use lib_dicepass_gen::PassphraseGenerator;

fn main() {
    let pg = PassphraseGenerator::new();
    println!("{}", pg.generate());
}
```
You should get something like: `pumpkin evolution exorcist wizardry eagerness`.

### About
lib_dicepass_gen uses the EFF's "short word list" (which can be found
[here](https://www.eff.org/files/2016/09/08/eff_short_wordlist_2_0.txt)) as
its default word-list and simulates dice rolls against that list to generate
pass-phrases.

The default pass-phrase length is 5, but custom length pass-phrases can be
generated like so:
```rust
fn main() {
    let pg = PassphraseGenerator::new();
    println!("{}", pg.generate_with_length(10));
}
```

Future versions will allow custom word-lists to be used as well.
