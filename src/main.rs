use std::{env, fmt::Display, process::exit};

mod pattern;
mod state;
mod word_filter;

use arrayvec::{ArrayString, ArrayVec};

use crate::{pattern::Pattern, word_filter::WordFilter};

const FILE: &'static str = include_str!("../list.txt");
const LOWER_CASE_A_VALUE: u8 = b'a';
const SPACE_VALUE: u8 = b' ';
const UNDERSCORE_VALUE: u8 = b'_';

const message_de_rappel: &str =
    "Rappel quand à l'utilisation de la commande: /tusmo2 <nombre de lettre> <premiére lettre>";

fn main() {
    let vars: Vec<String> = std::env::args().skip(1).collect();

    let nb_letter = vars[0].as_str().parse::<u8>().unwrap_or_else(|e| {
        eprintln!("Une erreur est survenue lors du parsing du nombre de lettre:\n{e}");
        eprintln!("{}", message_de_rappel);
        exit(1);
    });
    let _words = Words::load(len);
    if nb_letter != 5 {}
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
