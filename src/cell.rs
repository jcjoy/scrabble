/*
cell.rs
Implementation of cell for Scrabble board
*/ 

use crate::tile::Tile;

pub enum Bonus {
    None,
    DoubleLetter,
    DoubleWord,
    TripleLetter,
    TripleWord,
}
 
pub struct Cell {
    pub _tile: Option<Tile>, 
    pub _bonus: Bonus,
}

impl Cell {
    pub fn normal_cell() -> Cell {
        Cell {
            _tile: None,
            _bonus: Bonus::None,
        }
    }

    pub fn double_letter() -> Cell {
        Cell {
            _tile: None,
            _bonus: Bonus::DoubleLetter,
        }
    }

    pub fn double_word() -> Cell {
        Cell {
            _tile: None,
            _bonus: Bonus::DoubleWord,
        }
    }

    pub fn triple_letter() -> Cell {
        Cell {
            _tile: None,
            _bonus: Bonus::TripleLetter,
        }
    }

    pub fn triple_word() -> Cell {
        Cell {
            _tile: None,
            _bonus: Bonus::TripleWord,
        }
    }
}