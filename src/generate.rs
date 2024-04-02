use rand::Rng;

const LOWER_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGIT_CHARS: &str = "0123456789";
const SPECIAL_CHARS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub enum CharSets {
    Lower,
    Upper,
    Digit,
    Special,
}

impl CharSets {
    pub fn get_string(&self) -> &str {
        match self {
            CharSets::Lower => LOWER_CHARS,
            CharSets::Upper => UPPER_CHARS,
            CharSets::Digit => DIGIT_CHARS,
            CharSets::Special => SPECIAL_CHARS,
        }
    }
}

pub fn random_char(string: &str) -> Option<char> {
    if string.is_empty() {
        return None;
    }

    let index = rand::thread_rng().gen_range(0..string.len());
    string.chars().nth(index)
}

pub struct Unparsable;

pub fn create_string<F>(length: &usize, predicate: F) -> Result<String, Unparsable>
where
    F: Fn() -> char,
{
    let bytes: Vec<u8> = (0..*length).map(|_| predicate() as u8).collect();

    String::from_utf8(bytes).or(Err(Unparsable))
}
