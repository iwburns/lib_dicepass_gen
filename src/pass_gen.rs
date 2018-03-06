//!
//! Contains logic and structures relevant to passphrase generation.
//!

use std::collections::HashMap;
use rand::{OsRng, Rng};

use defaults;

///
/// Describes the possible word lists that can be used to generate a passphrase.
///
#[derive(Debug, Clone, Copy)]
pub enum WordList {
    ///
    /// The EFF's "long" word list.  This list contains 7776 words each of which provide ~12.9 bits
    /// of entropy.
    ///
    EffLong,

    ///
    /// The EFF's standard "short" word list.  These words are shorter than those contained in
    /// `EffLong`, but they provide less entropy per word (~10.3 bits/word).  This list contains
    /// 1296 words.
    ///
    EffShort,

    ///
    /// The EFF's "special" short word list.  This list provides similar entropy per word as
    /// `EffShort` (~10.3 bits/word), but has some extra bonuses:
    ///
    ///   1. Each word has a prefix of three unique characters
    ///   2. Each word is at least an edit distance of 3 from every other word
    ///
    /// This list also contains 1296 words.
    ///
    EffShort2,
}

///
/// Describes the length (in words) of a passphrase to generate.
///
#[derive(Debug, Clone, Copy)]
pub enum WordCount {
    ///
    /// This value depends on the `WordList` in use.
    ///
    Default,

    ///
    /// Specifies a custom length (in words) to use when generating a passphrase.
    ///
    Custom(u32),
}

impl From<WordCount> for Option<u32> {
    fn from(word_count: WordCount) -> Self {
        match word_count {
            WordCount::Default => None,
            WordCount::Custom(count) => Some(count),
        }
    }
}

impl From<Option<u32>> for WordCount {
    fn from(opt: Option<u32>) -> Self {
        match opt {
            None => WordCount::Default,
            Some(count) => WordCount::Custom(count),
        }
    }
}

///
/// A config object describing the rules for how to generate a passphrase.
///
#[derive(Debug, Clone, Copy)]
pub struct PassGenConfig {
    ///
    /// The word list to use when generating the passphrase.
    ///
    pub word_list: WordList,

    /// The length of the passphrase (in words) to generate.
    pub word_count: WordCount,
}

impl PassGenConfig {
    ///
    /// Helper function for creating `PassGenConfig` objects.
    ///
    /// Returns a `PassGenConfig` configured for the `EffLong` word list with `WordCount` number of
    /// words.
    ///
    pub fn from_eff_long(word_count: WordCount) -> PassGenConfig {
        PassGenConfig {
            word_list: WordList::EffLong,
            word_count,
        }
    }

    ///
    /// Helper function for creating `PassGenConfig` objects.
    ///
    /// Returns a `PassGenConfig` configured for the `EffShort` word list with `WordCount` number of
    /// words.
    ///
    pub fn from_eff_short(word_count: WordCount) -> PassGenConfig {
        PassGenConfig {
            word_list: WordList::EffShort,
            word_count,
        }
    }

    ///
    /// Helper function for creating `PassGenConfig` objects.
    ///
    /// Returns a `PassGenConfig` configured for the `EffShort2` word list with `WordCount` number
    /// of words.
    ///
    pub fn from_eff_short_2(word_count: WordCount) -> PassGenConfig {
        PassGenConfig {
            word_list: WordList::EffShort2,
            word_count,
        }
    }
}

///
/// Generates and returns a passphrase based on the given `PassGenConfig` object.
///
/// ```
/// use lib_dicepass_gen::*;
///
/// // generate a password containing 7 words from the `EffLong` word list
/// let config = PassGenConfig::from_eff_long(WordCount::Custom(7));
/// let pass = generate(config);
///
/// println!("{}", pass);
/// ```
///
pub fn generate(config: PassGenConfig) -> String {
    match config.word_list {
        WordList::EffLong => generate_passphrase(
            &defaults::eff_long::WORD_LIST,
            Option::from(config.word_count).unwrap_or(defaults::eff_long::WORD_COUNT),
            defaults::eff_long::ROLLS_PER_WORD,
        ),
        WordList::EffShort => generate_passphrase(
            &defaults::eff_short::WORD_LIST,
            Option::from(config.word_count).unwrap_or(defaults::eff_short::WORD_COUNT),
            defaults::eff_short::ROLLS_PER_WORD,
        ),
        WordList::EffShort2 => generate_passphrase(
            &defaults::eff_short_2::WORD_LIST,
            Option::from(config.word_count).unwrap_or(defaults::eff_short_2::WORD_COUNT),
            defaults::eff_short_2::ROLLS_PER_WORD,
        ),
    }
}

fn generate_passphrase(
    sequence_map: &'static HashMap<&'static str, &'static str>,
    length: u32,
    rolls_per_word: u32,
) -> String {
    let mut rng = OsRng::new().expect("couldn't get rng");
    let mut words = Vec::new();

    for _ in 0..length {
        let sequence = gen_sequence(rolls_per_word, &mut rng);
        if let Some(value) = sequence_map.get(sequence.as_str()) {
            words.push(*value);
        }
    }

    words.as_slice().join(" ")
}

fn gen_sequence(num_rolls: u32, rng: &mut OsRng) -> String {
    let mut sequence = String::new();
    for _ in 0..num_rolls {
        let roll = rng.gen_range(1u32, 7u32).to_string();
        sequence.push_str(&roll);
    }
    sequence
}

#[cfg(test)]
mod test {
    use super::*;
    use super::WordCount::*;

    #[test]
    fn gen_eff_long() {
        let pass_config = PassGenConfig::from_eff_long(Default);
        let pass = generate(pass_config);
        assert_eq!(
            pass.split_whitespace().count(),
            super::defaults::eff_long::WORD_COUNT as usize
        );
    }

    #[test]
    fn gen_eff_long_custom() {
        let word_count = 10;
        let pass_config = PassGenConfig::from_eff_long(Custom(word_count));
        let pass = generate(pass_config);
        assert_eq!(pass.split_whitespace().count(), word_count as usize);
    }

    #[test]
    fn gen_eff_short() {
        let pass_config = PassGenConfig::from_eff_short(Default);
        let pass = generate(pass_config);
        assert_eq!(
            pass.split_whitespace().count(),
            super::defaults::eff_short::WORD_COUNT as usize
        );
    }

    #[test]
    fn gen_eff_short_custom() {
        let word_count = 10;
        let pass_config = PassGenConfig::from_eff_short(Custom(word_count));
        let pass = generate(pass_config);
        assert_eq!(pass.split_whitespace().count(), word_count as usize);
    }

    #[test]
    fn gen_eff_short_2() {
        let pass_config = PassGenConfig::from_eff_short_2(Default);
        let pass = generate(pass_config);
        assert_eq!(
            pass.split_whitespace().count(),
            super::defaults::eff_short_2::WORD_COUNT as usize
        );
    }

    #[test]
    fn gen_eff_short_2_custom() {
        let word_count = 10;
        let pass_config = PassGenConfig::from_eff_short_2(Custom(word_count));
        let pass = generate(pass_config);
        assert_eq!(pass.split_whitespace().count(), word_count as usize);
    }
}

//
//fn parse_word_list(file_path: &str) -> std::io::Result<HashMap<String, String>> {
//    let mut word_list = File::open(file_path)?;
//
//    let mut file_contents = String::new();
//    word_list.read_to_string(&mut file_contents)?;
//
//    let line_count = file_contents.as_str().lines().count();
//    let map = HashMap::with_capacity(line_count);
//
//    let map = file_contents.lines().fold(map, |mut acc, line| {
//        let mut parts = line.split_whitespace();
//        let key = parts.next().unwrap_or("").to_string();
//        let value = parts.next().unwrap_or("").to_string();
//        acc.insert(key, value);
//        acc
//    });
//
//    return Ok(map);
//}
