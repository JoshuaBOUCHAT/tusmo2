use arrayvec::{ArrayString, ArrayVec};

use crate::{
    LOWER_CASE_A_VALUE,
    state::{CharState, State},
    word_filter::WordFilter,
};

pub struct Pattern {
    pattern: ArrayVec<CharState, 10>,
}

#[inline(always)]
pub fn get_available_and_needed(answer: &[u8], guess: &[u8]) -> ([u8; 26], [u8; 26]) {
    let mut available = [0; 26];
    let mut needed = [0; 26];

    for i in 0..answer.len() {
        let char_answer: u8 = answer[i];
        let char_guess: u8 = guess[i];
        if char_answer != char_guess {
            available[(char_guess - LOWER_CASE_A_VALUE) as usize] += 1;
            needed[(char_answer - LOWER_CASE_A_VALUE) as usize] += 1;
        }
    }
    (available, needed)
}

impl Pattern {
    pub fn from_answer_and_guess(answer: &str, guess: &str) -> Self {
        let len = answer.len();
        debug_assert_eq!(len, guess.len(), "size of answer and guess different");

        let answer = answer.as_bytes();
        let guess = guess.as_bytes();

        let mut res = ArrayVec::new_const();
        let (_availables, mut neededs) = get_available_and_needed(answer, guess);

        for i in 0..answer.len() {
            let char_answer = answer[i];
            let char_guess = guess[i];
            let index = (char_guess - LOWER_CASE_A_VALUE) as usize;
            let state = if char_answer == char_guess {
                State::Placed
            } else if neededs[index] > 0 {
                neededs[index] -= 1;
                State::ToPlaced
            } else {
                State::Wrong
            };

            res.push(CharState::new(state, char_guess));
        }

        return Self { pattern: res };
    }
}
impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer: ArrayString<12> = ArrayString::new_const();
        buffer.push('|');
        for char_state in &self.pattern {
            let repr_char = match char_state.state {
                State::Placed => (char_state.char as char).to_ascii_uppercase(),
                State::ToPlaced => char_state.char as char,
                State::Wrong => '#',
            };
            buffer.push(repr_char);
        }
        buffer.push('|');
        write!(f, "{}", buffer)
    }
}

#[test]
fn test_filter() {
    let filter = WordFilter::from_answer_and_guess(b"marines", b"aigrise");
    assert!(
        filter.filter(b"marines"),
        "Dois absolument passer les filtre ne dois jamais être capable de refuser la réponse"
    );
    assert!(
        !filter.filter(b"aigrise"),
        "Le filtre ne doit pas pouvoir accepter le mot refuser qui a permis de créer le filtre"
    );
    assert_eq!(filter.filter(b"ramsine"), false);

    let filter = WordFilter::from_answer_and_guess(b"eaae", b"eeie");
    assert!(
        !filter.filter(b"ezee"),
        "'e' est strictement limité à 2 occurrences (car trois 'e' on été essayé et un 'e' à repondu Wrong), mais 'ezee' en contient 3"
    );
}
