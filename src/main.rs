const FILE: &'static str = include_str!("../list.txt");
const LOWER_CASE_A_VALUE: u8 = 'a' as u8;
const SPACE_VALUE: u8 = ' ' as u8;
const UNDERSCORE_VALUE: u8 = '_' as u8;

fn main() {
    let len = 0;

    let words = Words::load(len);
}
#[derive(PartialEq, Eq, Debug, Clone, Copy, Default)]
enum State {
    #[default]
    Placed,
    ToPlaced,
    Wrong,
}

#[derive(Clone, Debug, Default)]
struct CharState {
    state: State,
    char: u8,
    strict: bool,
}
impl CharState {
    fn new(state: State, char: u8, strict: bool) -> Self {
        Self {
            state,
            char,
            strict,
        }
    }
}

struct Pattern {
    pattern: Vec<CharState>,
}

fn get_availabality_and_strictness(
    anwser: &[u8],
    guess: &[u8],
) -> Result<([i32; 26], [bool; 26]), String> {
    let mut available = [0; 26];
    let mut strict = [false; 26];
    for i in 0..anwser.len() {
        let char_pattern = anwser[i];
        let char_guess = guess[i];

        match char_pattern {
            c if c == char_pattern => {
                continue;
            }
            UNDERSCORE_VALUE => {
                available[(char_guess - LOWER_CASE_A_VALUE) as usize] += 1;
            }
            SPACE_VALUE => {
                strict[(char_guess - LOWER_CASE_A_VALUE) as usize] = true;
            }
            c => {
                return Err(format!(
                    "Imposible de lire le pattern car celui-ci est érroné mauvais char: {}",
                    c as char
                ));
            }
        }
    }
    Ok((available, strict))
}

impl Pattern {
    fn from_awnser_and_pattern(anwser: &str, guess: &str) -> Result<Self, String> {
        let len = anwser.len();
        if guess.len() != len {
            return Err(format!(
                "Pattern and guess size do not match {} : {}",
                guess.len(),
                len
            ));
        }

        let awnser = anwser.as_bytes();
        let guess = guess.as_bytes();

        let mut result = vec![CharState::default(); len];

        //permet de savoir les lettres disponible c'est a dire celle qui sont sont mal placé
        let (mut available, strict) = get_availabality_and_strictness(guess, awnser)?;
        for i in 0..len {
            let char_awnser = awnser[i];
            let char_guess = guess[i];
            result[i].char = char_guess;

            match char_awnser {
                c if c == char_awnser => {
                    continue;
                }
                UNDERSCORE_VALUE => {
                    let maps_index = (char_guess - LOWER_CASE_A_VALUE) as usize;

                    if available[maps_index] > 0 {
                        available[maps_index] -= 1;

                        result[i].state = State::ToPlaced;
                        result[i].strict = strict[maps_index];
                    } else {
                        result[i].char = char_guess;
                    }
                }
                SPACE_VALUE => {}
                c => unsafe { std::hint::unreachable_unchecked() },
            }
        }

        let strict = [false; 26];
    }
}

#[derive(Debug)]
struct Words {
    inner: Vec<&'static [u8]>,
}
impl Words {
    fn load(word_size: usize) -> Self {
        let inner = FILE
            .lines()
            .filter(|&s| s.len() == word_size)
            .map(|s| s.as_bytes())
            .collect();
        Self { inner }
    }
}
