#![feature(iter_array_chunks)]

use std::{env, fmt::Display, io::stdin, process::exit};

mod pattern;
mod state;
mod word_filter;
mod words;

use arrayvec::{ArrayString, ArrayVec};

use crate::{pattern::Pattern, word_filter::WordFilter, words::Words};

const FILE: &'static str = include_str!("../list.txt");
const LOWER_CASE_A_VALUE: u8 = b'a';
const SPACE_VALUE: u8 = b' ';
const UNDERSCORE_VALUE: u8 = b'_';

const MESSAGE_DE_RAPPEL: &str =
    "Rappel quand à l'utilisation de la commande: /tusmo2 <nombre de lettre> <premiére lettre ";

fn main() {
    let vars: Vec<String> = std::env::args().skip(1).collect();
    println!("var len: {}", vars.len());
    if vars.len() < 1 {
        eprintln!("Aucun argument n'as été donnée en paramètre !");
        return;
    }

    let nb_letter = vars[0].as_str().parse::<usize>().unwrap_or_else(|e| {
        eprintln!("Une erreur est survenue lors du parsing du nombre de lettre:\n{e}");
        eprintln!("{}", MESSAGE_DE_RAPPEL);
        exit(1);
    });
    let mut words: Words<'static> = Words::load(nb_letter);
    println!("Number of words before filter: {}", words.len());

    if vars.len() > 1 {
        let Some(first_letter) = vars[1].chars().nth(0) else {
            eprintln!("Impossible d'obtenir la première lettre du mot");
            eprintln!("{}", MESSAGE_DE_RAPPEL);
            return;
        };
        println!("Filtering words with first letter {}", first_letter);
        let filter_first_letter = WordFilter::from_first_letter(first_letter as u8);
        words.filter(&filter_first_letter);
    }
    println!("Now starting !");
    while let Some(best_word) = words.get_optimal() {
        println!("The optimal word is: {}", best_word);
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer.trim_end_matches('\n')
    }
}
