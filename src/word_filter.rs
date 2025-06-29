use arrayvec::ArrayVec;

use crate::{LOWER_CASE_A_VALUE, pattern::get_available_and_needed, words::Words};

#[derive(Default, Debug, Clone)]
pub struct WordFilter {
    ///letter, letter_number, strictness
    count_filter: ArrayVec<(u8, u8, bool), 10>,

    ///position, letter, wanted
    position_filter: ArrayVec<(usize, u8, bool), 10>,
}
impl WordFilter {
    pub fn from_answer_and_guess(answer: &[u8], guess: &[u8]) -> Self {
        let (availables, neededs) = get_available_and_needed(answer, guess);
        let mut processeds: [bool; 26] = [false; 26];
        let mut word_filter = WordFilter::default();
        for i in 0..answer.len() {
            let answer_char = answer[i];
            let guess_char = guess[i];
            let are_letter_equal = answer_char == guess_char;
            word_filter
                .position_filter
                .push((i, guess_char, are_letter_equal));

            let index = (guess_char - LOWER_CASE_A_VALUE) as usize;

            //le count de cette lettre est deja proccess
            if processeds[index] {
                continue;
            }

            processeds[index] = true;

            //si on a au moin une lettre Wrong alors on connais le nombre exact
            //on sais si le nombre est strict
            let strict = availables[index] > neededs[index];
            word_filter
                .count_filter
                .push((guess_char, neededs[index], strict));
        }

        word_filter
    }
    pub fn from_pattern_str(pattern_str: &str) -> Self {
        let pattern = pattern_str.as_bytes();
        let mut count
    }

    pub fn filter(&self, to_filter: &[u8]) -> bool {
        for &(position, val, is_wanted) in &self.position_filter {
            if (to_filter[position] == val) != is_wanted {
                return false;
            }
        }
        let counts = get_counts(to_filter);

        for &(letter, needed_count, strict) in &self.count_filter {
            let count = counts[(letter - LOWER_CASE_A_VALUE) as usize];
            if strict {
                if count != needed_count {
                    return false;
                }
            } else {
                if count < needed_count {
                    return false;
                }
            }
        }

        return true;
    }
    pub fn from_first_letter(first_letter: u8) -> Self {
        let mut res = Self::default();
        res.position_filter.push((0, first_letter, true));
        res.count_filter.push((first_letter, 1, false));
        res
    }
}

fn get_counts(word: &[u8]) -> [u8; 26] {
    let mut counts = [0u8; 26];
    for &letter in word {
        counts[(letter - LOWER_CASE_A_VALUE) as usize] += 1;
    }
    counts
}
mod test {
    use crate::word_filter::WordFilter;

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
        let filter = WordFilter::from_first_letter(b'a');

        assert!(
            !filter.filter(b"ba"),
            "should filter as ba do not start with letter 'a' "
        );
        dbg!(filter);
    }
}
