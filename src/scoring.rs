use board;
use tile;
use cell;
use std::vec::Vec;

pub fn score_word(word: Vec<cell>&) -> usize {
    let letter_multiplier: usize = match word[0]._bonus {
         cell::Bonus::DoubleLetter => 2,
         cell::Bonus::TripleLetter => 3,
         _ => 1,
     };
     let word_multiplier: usize = match word[0]._bonus {
         cell::Bonus::DoubleWord => 2,
         cell::Bonus::TripleWord => 3,
         _ => 1,
     }
     let score = match word[0]._tile {
         Some(t) => word_multiplier * (letter_multiplier * t.value + score_word(&word[1..])),
         None => panic!("EMPTY TILE CANNOT BE SCORED!"),
     }
}