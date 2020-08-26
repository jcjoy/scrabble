// tile.rs
// Implementation of tile for basic scrabble game

#[derive(Debug)]
pub struct Tile {
    pub letter : char,
    pub value : usize,
}

impl Tile {
    pub fn new(letter: char) -> Tile {
        Tile {
            letter: letter,
            value: get_value(letter),
        }
    }
}

fn get_value(letter: char) -> usize {
        match letter {
            'a' => 1,
            'b' => 3,
            'c' => 3,
            'd' => 2, 
            'e' => 1,
            'f' => 4,
            'g' => 2,
            'h' => 4,
            'i' => 1,
            'j' => 8,
            'k' => 5,
            'l' => 4, 
            'm' => 3, 
            'n' => 1,
            'o' => 1,
            'p' => 3,
            'q' => 10,
            'r' => 1,
            's' => 1,
            't' => 1,
            'u' => 1,
            'v' => 4,
            'w' => 4,
            'x' => 8,
            'y' => 4,
            'z' => 10,
            _ => panic!("UNKNOWN LETTER!")
        }
    }
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let exp = Tile {
            letter: 'a',
            value: 1,
        };
        let act = Tile::new('a');
        assert_eq!(act.value, exp.value);
        assert_eq!(act.letter, 'a');
    }

    #[test]
    fn test_z() {
        let exp = Tile {
            letter: 'z',
            value: 10,
        };
        let act = Tile::new('z');
        assert_eq!(act.value, exp.value);
        assert_eq!(act.letter, 'z');
    }
}