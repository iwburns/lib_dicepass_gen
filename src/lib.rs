#[macro_use]
extern crate lazy_static;
extern crate rand;

mod defaults;
pub mod pass_gen;

pub use pass_gen::PassphraseGenerator;

#[cfg(test)]
mod tests {
    use ::defaults;

    #[test]
    fn default_word_list() {
        let wordlist = &defaults::WORD_LIST;
        assert_eq!(wordlist.get("1111").unwrap(), &"aardvark");
        assert_eq!(wordlist.get("2222").unwrap(), &"datebook");
        assert_eq!(wordlist.get("3333").unwrap(), &"grill");
        assert_eq!(wordlist.get("4444").unwrap(), &"nebula");
        assert_eq!(wordlist.get("5555").unwrap(), &"siesta");
        assert_eq!(wordlist.get("6666").unwrap(), &"zucchini");
    }
}
