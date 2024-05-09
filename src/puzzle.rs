use std::fs;
use rand::seq::SliceRandom;

pub enum PuzzleError {
    BoardInitialized
}

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

#[derive(PartialEq)]
pub struct Piece {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
    place: Position,
    rotation: Rotation
}

pub fn determine_position(top: u32, right: u32) -> Position{
    if top == 0 {
        if right == 0 {
            return Position::Corner;
        } else {
            return Position::Border;
        }
    } else {
        return Position::Middle;
    }
}

#[derive(PartialEq)]
pub struct Puzzle {
    height: u32,
    width: u32,
    pieces: Vec<Piece>,
    board: Option<Vec<Vec<u32>>>
}

impl Puzzle {

    /// Initialize a new puzzle from an input file ("pieces_NNxNN").
    /// The board is kept empty for now ; run init() to randomly initialise it.
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
            pieces_list.push(Piece {top: sides[0], right: sides[1], bottom: sides[2], left: sides[3], place: determine_position(sides[0], sides[1]), rotation: Rotation::NoTurn});
        }
        
        return Puzzle {height: dimensions[0], width: dimensions[1], pieces: pieces_list, board: None};
    }

    /// Make a puzzle by randomly putting the pieces at the right places
    pub fn init(mut self) -> Result<(), PuzzleError>{

        match self.board {
            None => {
                let board: Vec<Vec<u32>> = vec![vec![0; self.height.try_into().unwrap()]; self.width.try_into().unwrap()];

                let mut corners: Vec<u32> = Vec::new();
                let mut borders: Vec<u32> = Vec::new();
                let mut middles: Vec<u32> = Vec::new();
        
                let mut count:u32 = 0;
                for i in 0..(new_puzzle.height as usize) {
                    for j in 0..(new_puzzle.width as usize) {
                        if i == 0 || i == (new_puzzle.height as usize) {
                            if j == 0 || j == (new_puzzle.width as usize) {
                                // On est sur les index des coins
                                corners.push(count);
                                count += 1;
                            } else {
                                // On est sur les index des bords
                                borders.push(count);
                                count += 1;
                            }
                        } else {
                            if j == 0 || j == (new_puzzle.width as usize) {
                                // On est sur les index des bords
                                borders.push(count);
                                count += 1;
                            } else {
                                // On est sur les index des milieux
                                middles.push(count);
                                count += 1;
                            }
                        }
                    } 
                }
        
                // Placement des coins
        
                let pick = *corners.choose(&mut rand::thread_rng()).unwrap();
                new_puzzle.board[0][0] = pick;
        
                let pick = *corners.choose(&mut rand::thread_rng()).unwrap();
                new_puzzle.board[0][new_puzzle.width as usize] = pick;
        
                let pick = *corners.choose(&mut rand::thread_rng()).unwrap();
                new_puzzle.board[new_puzzle.height as usize][0] = pick;
        
                let pick = *corners.choose(&mut rand::thread_rng()).unwrap();
                new_puzzle.board[new_puzzle.height as usize][new_puzzle.width as usize] = pick;
        
                // Placement des bords
                for i in 0..(new_puzzle.height as usize) {
                    match i {
                        0 => {
        
                        },
                        (new_puzzle.height as usize) => {
                            prinln!();
                        },
                        _ => {
                            let pick = *borders.choose(&mut rand::thread_rng()).unwrap();
                            new_puzzle.board[0][0] = pick;
        
                            let pick = *borders.choose(&mut rand::thread_rng()).unwrap();
                            new_puzzle.board[0][new_puzzle.width as usize] = pick;
                        }
                    }
                }
        
                // Placement des milieux
        
                return Ok();
            },
            Some(board) => {
                return Err(PuzzleError::BoardInitialized);
            } 
        }
    }

    /// Compute the current puzzle fitness, aka the number of matching edges inside the puzzle.
    pub fn fitness(&self) -> u32 {
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

    pub fn max_fitness(&self) -> u32 {
        2*(self.height-1)*self.width
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
