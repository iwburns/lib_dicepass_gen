use std::collections::HashMap;
use rand::{Rng, OsRng};
use ::defaults;

pub struct PassphraseGenerator {
    word_count: u32,
    rolls_per_word: u32,
    sequence_map: &'static HashMap<&'static str, &'static str>,
}

impl PassphraseGenerator {
    pub fn new() -> PassphraseGenerator {
        PassphraseGenerator {
            word_count: defaults::WORD_COUNT,
            rolls_per_word: defaults::ROLLS_PER_WORD,
            sequence_map: &defaults::WORD_LIST,
        }
    }

    pub fn generate(&self) -> String {
        self.generate_with_length(self.word_count)
    }

    pub fn generate_with_length(&self, length: u32) -> String {
        let mut rng = OsRng::new().expect("couldn't get rng");
        let mut words = Vec::with_capacity(length as usize);

        for _ in 0..length {
            let sequence = gen_sequence(self.rolls_per_word, &mut rng);
            if let Some(value) = self.sequence_map.get(sequence.as_str()) {
                words.push(*value);
            }
        }

        words.as_slice().join(" ")
    }
}

impl Default for PassphraseGenerator {
    fn default() -> Self {
        PassphraseGenerator::new()
    }
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
mod tests {
    use super::*;

    #[test]
    fn gen_sequence() {
        let mut rng = OsRng::new().expect("couldn't get OsRng.");
        let seq_length = 5;
        let sequence = super::gen_sequence(seq_length, &mut rng);
        assert_eq!(sequence.len(), seq_length as usize);
    }

    #[test]
    fn default_pass_phrase_length() {
        let phrase = PassphraseGenerator::new().generate();
        assert_eq!(phrase.split(" ").count(), 8);
    }

    #[test]
    fn custom_phrase_length() {
        let length = 7;
        let phrase = PassphraseGenerator::new().generate_with_length(length);
        assert_eq!(phrase.split(" ").count(), length as usize);
    }
}
