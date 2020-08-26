mod board;
mod cell;
mod tile;

fn main() {

    let mut board = board::Board::scrabble_board();
    board.squares[0]._tile =  Some(tile::Tile{letter: 't', value: 1});
    for i in 0..224 {
    match &board.squares[i]._tile {
        Some(e) => println!("Letter {}", e.letter),
        None => println!("None"),
    }}
    println!("Hello, world!");
}