use std::vec::Vec;
use crate::cell::Cell;

pub struct Board{
    pub squares: Vec<Cell> 
}

impl Board {
    pub fn scrabble_board() -> Board {
        let n = ||-> Cell {Cell::normal_cell()};
        let dl = ||-> Cell {Cell::double_letter()};
        let dw = ||-> Cell {Cell::double_word()};
        let tl = ||-> Cell {Cell::triple_letter()};
        let tw = ||-> Cell {Cell::triple_word()};

        let b = Board {
            squares:
            vec!(
                tw(),  n(),  n(), dl(), n(),  n(),  n(), tw(),  n(),  n(),  n(), dl(),  n(),  n(),  tw(),
                n(),  dw(),  n(),  n(), n(), tl(),  n(),  n(),  n(), tl(),  n(),  n(),  n(), dw(),   n(),
                n(),   n(), dw(),  n(), n(),  n(), dl(),  n(), dl(),  n(),  n(),  n(), dw(),  n(),   n(),
                dl(),  n(),  n(), dw(), n(),  n(),  n(), dl(),  n(),  n(),  n(), dw(),  n(),  n(),  dl(),
                n (),  n(),  n(),  n(),dw(),  n(),  n(),  n(),  n(),  n(), dw(),  n(),  n(),  n(),   n(),
                 n(), tl(),  n(),  n(), n(), tl(),  n(),  n(),  n(), tl(),  n(),  n(),  n(), tl(),   n(),
                 n(),  n(), dl(),  n(), n(),  n(),  dl(), n(), dl(),  n(),  n(),  n(), dl(),  n(),   n(),
                tw(),  n(),  n(), dl(), n(),  n(),   n(),dw(),  n(),  n(),  n(), dl(),  n(),  n(),  tw(),
                n(),  n(), dl(),  n(), n(),  n(),  dl(), n(), dl(),  n(),  n(),  n(), dl(),  n(),   n(),
                 n(), tl(),  n(),  n(), n(), tl(),  n(),  n(),  n(), tl(),  n(),  n(),  n(), tl(),   n(),
                n (),  n(),  n(),  n(),dw(),  n(),  n(),  n(),  n(),  n(), dw(),  n(),  n(),  n(),   n(),
                dl(),  n(),  n(), dw(), n(),  n(),  n(), dl(),  n(),  n(),  n(), dw(),  n(),  n(),  dl(),
                n(),   n(), dw(),  n(), n(),  n(), dl(),  n(), dl(),  n(),  n(),  n(), dw(),  n(),   n(),
                n(),  dw(),  n(),  n(), n(), tl(),  n(),  n(),  n(), tl(),  n(),  n(),  n(), dw(),   n(),
                tw(),  n(),  n(), dl(), n(),  n(),  n(), tw(),  n(),  n(),  n(), dl(),  n(),  n(),  tw(),
            )
        };
        b
    }

}
