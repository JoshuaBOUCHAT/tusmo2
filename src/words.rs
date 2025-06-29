use std::{
    ops::{Index, IndexMut},
    str::from_utf8,
    sync::{RwLock, atomic::AtomicI32, atomic::Ordering::SeqCst},
    usize,
};

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use crate::{FILE, word_filter::WordFilter};

#[derive(Debug)]
pub struct Words<'a> {
    inner: Vec<&'a [u8]>,
}
impl<'a> Index<usize> for Words<'a> {
    type Output = &'a [u8];
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}
impl<'a> IndexMut<usize> for Words<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<'a> Words<'a> {
    pub fn load(word_size: usize) -> Self {
        let inner = FILE
            .lines()
            .filter(|&s| s.len() == word_size)
            .map(|s| s.as_bytes())
            .collect();
        Self { inner }
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn filter(&mut self, word_filter: &WordFilter) {
        //let mut i = self.inner.len();
        /*while i > 0 {
            i -= 1;
            if word_filter.filter(&self.inner[i]) {
                self.inner.swap_remove(i);
            }
        }*/
        self.inner.retain(|&s| word_filter.filter(s));
    }
    pub fn get_score(&self, index: usize, max_count: usize) -> usize {
        let test_word = self[index];
        let mut total_count = 0;
        let len = self.len();

        if index > 0 {
            for i in 0..index {
                let filter = WordFilter::from_answer_and_guess(test_word, self[i]);
                total_count += self.inner.iter().filter(|&&s| filter.filter(s)).count();
                if total_count > max_count {
                    return total_count;
                }
            }
        }
        for i in (index + 1)..len {
            let filter = WordFilter::from_answer_and_guess(test_word, self[i]);
            total_count += self.inner.iter().filter(|&&s| filter.filter(s)).count();
            if total_count > max_count {
                return total_count;
            }
        }
        total_count
    }

    pub fn get_optimal(&self) -> Option<&str> {
        let len = self.len();
        println!("Searching for the best out of {}", len);

        if len == 0 {
            return None;
        }
        let min = RwLock::new(self[0]);
        let min_score = RwLock::new(self.get_score(0, usize::MAX));
        let counter = AtomicI32::new(0);
        (1..len)
            .array_chunks::<10>()
            .into_iter()
            .par_bridge()
            .for_each(|i| {
                let mut local_min = *min_score.read().unwrap();
                for j in 0..i.len() {
                    let score = self.get_score(j, local_min);

                    if score < *min_score.read().unwrap() {
                        println!(
                            "The new best word find is {} with score in average of: {}",
                            from_utf8(&self[j]).unwrap(),
                            score as f64 / self.len() as f64
                        );
                        local_min = score;
                        *min_score.write().unwrap() = score;
                        *min.write().unwrap() = self[j];
                    }
                }
                counter.fetch_add(10, SeqCst);
                let counter_val = counter.load(std::sync::atomic::Ordering::Relaxed);
                if counter_val % 100 == 0 {
                    println!(
                        "step {} over {}",
                        counter_val / 100,
                        (self.len() + 99) / 100
                    );
                }
            });

        from_utf8(*min.read().unwrap()).ok()
    }
}
#[test]
fn test_words() {
    let mut words = Words::load(2);
    assert!(
        words.len() == 77,
        "La list contient 77 mots de deux lettres !"
    );

    let filter = WordFilter::from_first_letter(b'a');
    words.filter(&filter);

    assert!(
        words.len() == 7,
        "il existe 7 mot de 2 lettres commençant par a ! ici on as: {:?}",
        words
            .inner
            .iter()
            .map(|b| from_utf8(b).unwrap())
            .collect::<Vec<&str>>()
    );

    let expecteds = ["aa", "ah", "ai", "an", "as", "au", "ay"];

    for expected in expecteds {
        assert!(words.inner.contains(&expected.as_bytes()));
    }
}
