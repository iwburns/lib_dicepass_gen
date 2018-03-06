# lib_dicepass_gen

A library for generating pass-phrases from dice-rolls against known word lists.

### Disclaimer
This project is currently a work-in-progress.  I make no guarantees about its correctness or
security.  I hope to provide such guarantees in the future.

### Usage
```rust
use lib_dicepass_gen::*;
    
fn main() {
    use lib_dicepass_gen::WordCount::*;
    
    let config = PassGenConfig::from_eff_long(Default);
    let pass = generate(config);
    
    println!("{}", pass);
    // should result in something like:
    // "resubmit gallon faceted duplex octagon"
}
```
Custom length pass-phrases can be generated like so:
```rust
use lib_dicepass_gen::*;
    
fn main() {
    use lib_dicepass_gen::WordCount::*;
    
    let config = PassGenConfig::from_eff_long(Custom(7));
    let pass = generate(config);
    
    println!("{}", pass);
    // should result in something like:
    // "untouched scouting pronto gauging tripping resume derived"
}
```

### About
`lib_dicepass_gen` uses the [Diceware](http://world.std.com/%7Ereinhold/diceware.html) method
of passphrase generation where dice are rolled to pick words from a known dictionary.

This library uses one of the EFF's word lists as a dictionary and then simulates dice rolls to
randomly select enough words to generate a secure passphrase.  There are currently three
word lists available to choose from when generating a passphrase (those provided by the EFF
[here](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases)), but more may be
added in the future.
