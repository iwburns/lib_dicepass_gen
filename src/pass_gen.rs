use std::collections::HashMap;
use rand::{OsRng, Rng};
use defaults;

#[derive(Debug, Clone, Copy)]
pub enum WordList {
    EffLong,
    EffShort,
    EffShort2,
}

#[derive(Debug, Clone, Copy)]
pub enum WordCount {
    Default,
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

#[derive(Debug, Clone, Copy)]
pub struct PassGenConfig {
    pub word_list: WordList,
    pub word_count: WordCount,
}

impl PassGenConfig {
    pub fn from_eff_long(word_count: WordCount) -> PassGenConfig {
        PassGenConfig {
            word_list: WordList::EffLong,
            word_count,
        }
    }

    pub fn from_eff_short(word_count: WordCount) -> PassGenConfig {
        PassGenConfig {
            word_list: WordList::EffShort,
            word_count,
        }
    }

    pub fn from_eff_short_2(word_count: WordCount) -> PassGenConfig {
        PassGenConfig {
            word_list: WordList::EffShort2,
            word_count,
        }
    }
}

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
    let mut words = Vec::with_capacity(length as usize);

    for _ in 0..length {
        let sequence = gen_sequence(rolls_per_word, &mut rng);
        if let Some(value) = sequence_map.get(sequence.as_str()) {
            words.push(*value);
        }
    }

    words.as_slice().join(" ")
}

fn gen_sequence(num_rolls: u32, rng: &mut OsRng) -> String {
    let mut sequence = String::with_capacity(num_rolls as usize);
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
