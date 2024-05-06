use std::fs;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Rotation {
    NoTurn,
    QuarterTurn,
    HalfTurn,
    ThreeQuarterTurn
}

#[derive(PartialEq)]
pub enum Position {
    Unknown,
    Corner,
    Border,
    Middle
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub struct Piece {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
    place: Position,
    rotation: Rotation
}

#[allow(dead_code)]
pub struct Puzzle {
    height: u32,
    width: u32,
    pieces: Vec<Piece>,
    board: Vec<Vec<u32>>
}

#[allow(dead_code)]
impl Puzzle {

    /// Initialize a new puzzle from an input file ("pieces_NNxNN").
    pub fn new(pieces_file: &str) -> Puzzle {
        let contents = fs::read_to_string(pieces_file).unwrap();
        let lines: Vec<&str> = contents.split('\n').collect();

        // Read the first line to get puzzle dimensions
        let dimensions: Vec<u32> = lines[0].split(' ').map(|x| x.parse::<u32>().unwrap()).collect();

        // Read the other lines to get the pieces
        let mut pieces_list: Vec<Piece> = Vec::new();
        let number_of_pieces = lines.len();
        for i in 0..(number_of_pieces-1) {
            let sides: Vec<u32> = lines[i+1].split(' ').map(|x| x.parse::<u32>().unwrap()).collect();
            pieces_list.push(Piece {top: sides[0], right: sides[1], bottom: sides[2], left: sides[3], place: Position::Unknown, rotation: Rotation::NoTurn});
        }

        // Creates the board matrix
        let board: Vec<Vec<u32>> = vec![vec![0; dimensions[1].try_into().unwrap()]; dimensions[0].try_into().unwrap()];
        
        return Puzzle {height: dimensions[0], width: dimensions[1], pieces: pieces_list, board: board};
    }

    /// Initialises a puzzle correctly, by putting the corner pieces at the corners, the border pieces at the borders and the middle pieces in the middle.
    /// The puzzle is not correct yet.
    pub fn init(&mut self) {
        let mut count: u32 = 0;
        for i in 0..(self.height) {
            for j in 0..(self.width) {
                self.board[i as usize][j as usize] = count;
                count = count + 1
            }
        }
    }

    /// Compute the current puzzle fitness, aka the number of matching edges inside the puzzle.
    pub fn fitness(self) -> u32 {
        let mut score: u32 = 0;

        // Vertically
        for i in 0..((self.height) as usize) {
            for j in 0..((self.width - 1) as usize) {
                if self.pieces[self.board[i][j] as usize].right == self.pieces[self.board[i][j+1] as usize].left {
                    score += 1;
                }
            }
        }

        // Horizontally
        for i in 0..((self.height - 1) as usize) {
            for j in 0..(self.width as usize) {
                if self.pieces[self.board[i][j] as usize].bottom == self.pieces[self.board[i+1][j] as usize].top {
                    score += 1;
                }
            }
        }

        return score;
    }

    /// Prints the content of the puzzle to the standart output.
    pub fn show(&self) {
        println!("Dimensions : {} x {}", self.height, self.width);
        println!();

        println!("Liste des pièces (dans l'ordre) :");
        for piece in &self.pieces {
            println!("{} {} {} {}", piece.top, piece.right, piece.bottom, piece.left);
        }

        println!();

        println!("Position des pièces sur le plateau :");
        for row in 0..(self.board.len()) {
            for column in 0..(self.board[row].len()) {
                print!("{}  ", self.board[row][column]);
            }
            println!()
        }
        
        println!();
    }

}

pub fn hello() {
    println!("Hello from puzzle.rs !");
}

#[cfg(test)]
mod tests {
    use crate::puzzle::*;

    #[test]
    fn create_puzzle_from_file() {
        let puzzle = Puzzle::new("./pieces_set/pieces_04x04.txt");

        let theoretical_pieces: Vec<Piece> = vec![
            Piece {top: 0, right: 0, bottom: 1, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 0, bottom: 1, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 0, bottom: 2, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 0, bottom: 2, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 1, bottom: 3, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 1, bottom: 4, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 1, bottom: 5, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 1, bottom: 5, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 2, bottom: 3, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 2, bottom: 4, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 2, bottom: 4, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 0, right: 2, bottom: 5, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 3, right: 3, bottom: 4, left: 4, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 3, right: 3, bottom: 5, left: 5, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 3, right: 4, bottom: 5, left: 4, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {top: 3, right: 5, bottom: 5, left: 5, place: Position::Unknown, rotation:Rotation::NoTurn},
        ];

        // let theoretical_board: Vec<Vec<u32>> = vec![
        //     vec![0, 1, 2, 3],
        //     vec![4, 5, 6, 7], 
        //     vec![8, 9, 10, 11],
        //     vec![12, 13, 14, 15]
        // ];

        assert!(puzzle.height == 4);
        assert!(puzzle.width == 4);
        assert!(puzzle.pieces.len() == theoretical_pieces.len());

        // for i  in 1..theoretical_pieces.len() {
        //     assert!(puzzle.pieces[i] == theoretical_pieces[i])
        // }

        // assert!(puzzle.board == theoretical_board);

    }
}
