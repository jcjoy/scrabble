/*
validation.rs
Module for validating scrabble plays

Jim Joy
2020
*/

use crate::board;
use crate::tile;
use crate::cell;

use std::vec::Vec;

type Coord = (i32, i32);
type Word = Vec<tile::Tile>;

pub fn valid_first_play(start: Coord, end: Coord) -> bool {
    
    let (row_start, col_start) = start;
    let (row_end, col_end) = end;

    // The center coordinate must be in the range of the valid coordinates
    (row_start <= 7) && (7 <= row_end) && (col_start <= 7) && (7 <= col_end) 
}

fn valid_word(word: Word) -> bool {

    let word_str: String = word.into_iter().map(|t| t.letter).collect();
    //dictionary::is_word(&word_str)
    true
}

fn valid_rowwise_play(start: Coord, end: Coord) -> bool {

    let (row_start, _) = start;
    let (row_end, _) = end;

    row_start == row_end
}

fn valid_column_play(start: Coord, end: Coord) -> bool {

    let (_, col_start) = start;
    let (_, col_end) = end;

    col_start == col_end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_first() {
        let start = (7,3);
        let end = (7, 8);
        assert!(valid_first_play(start, end));
    }

    #[test]
    fn test_invalid_first(){
        let start = (0,0);
        let end = (0, 6);
        assert!(!valid_first_play(start, end));
    }
}