use phf;
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

impl From<Option<u32>> for WordCount {
    fn from(opt: Option<u32>) -> Self {
        match opt {
            None => WordCount::Default,
            Some(count) => WordCount::Custom(count),
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
        _ => "".to_string()
    }
}

fn generate_passphrase(
    sequence_map: &'static phf::Map<&'static str, &'static str>,
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
