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
    number: u32,
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
    pieces: Vec<Vec<Piece>>,
}

#[allow(dead_code)]
impl Puzzle {

    /// Initialize a new puzzle from an input file.
    pub fn new(pieces_file: &str) -> Puzzle {
        let contents = fs::read_to_string(pieces_file).unwrap();
        let lines: Vec<&str> = contents.split('\n').collect();

        // Read the first line to get puzzle dimensions
        let dimensions: Vec<u32> = lines[0].split(' ').map(|x| x.parse::<u32>().unwrap()).collect();

        // Read the other lines to get the pieces
        let mut pieces_list: Vec<Vec<Piece>> = Vec::new();
        // let number_of_pieces: usize = lines.len() - 2;

        for _i in 0..dimensions[0] {
            pieces_list.push(Vec::new());
        }

        let mut lines_count: usize = 1;
        for i in 0..((dimensions[0]) as usize) {
            for _j in 0..((dimensions[1]) as usize) {
                let sides: Vec<u32> = lines[lines_count].split(' ').map(|x| x.parse::<u32>().unwrap()).collect();
                pieces_list[i].push(Piece {number: (lines_count - 1).try_into().unwrap(), top: sides[0], right: sides[1], bottom: sides[2], left: sides[3], place: Position::Unknown, rotation: Rotation::NoTurn});
                lines_count += 1;
            }
           
        }
        
        return Puzzle {height: dimensions[0], width: dimensions[1], pieces: pieces_list};
    }

    /// Initialises a puzzle correctly, by putting the corner pieces at the corners, the border pieces at the borders and the middle pieces in the middle.
    /// The puzzle is not correct yet.
    pub fn init(self) {
        todo!();
    }

    /// Compute the current puzzle fitness, aka the number of matching edges inside the puzzle.
    pub fn fitness(self) -> u32 {
        let mut score = 0;

        // Look at the horizontal borders
        for i in 0..((self.height - 1) as usize) {
            for j in 0..((self.width - 2) as usize) {
                if self.pieces[i][j].bottom == self.pieces[i][j+1].top {
                    score += 1;
                }
            }
        }

        // Look at the vertical borders
        for i in 0..((self.height - 1) as usize) {
            for j in 0..((self.width - 2) as usize) {
                if self.pieces[i][j].right == self.pieces[i+1][j].left {
                    score += 1;
                }
            }
        }

        return score;
    }

    /// Prints the content of the puzzle to the standart output.
    pub fn show(self) {
        println!("Dimensions : {} x {}", self.height, self.width);
        println!();

        println!("Liste des pièces (dans l'ordre) :");
        for row in 0..((self.height) as usize) {
            for column in 0..((self.width) as usize) {
                println!("{} {} {} {}", &self.pieces[row][column].top, &self.pieces[row][column].right, &self.pieces[row][column].bottom, &self.pieces[row][column].left);
            }
        }

        println!();

        println!("Position des pièces sur le plateau :");
        for row in 0..((self.height) as usize) {
            for column in 0..((self.width) as usize) {
                print!("{}  ", &self.pieces[row][column].number);
            }
            println!()
        }

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
        let puzzle = Puzzle::new("pieces_04x04.txt");

        let theoretical_pieces: Vec<Piece> = vec![
            Piece {number: 0, top: 0, right: 0, bottom: 1, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 1, top: 0, right: 0, bottom: 1, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 2, top: 0, right: 0, bottom: 2, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 3, top: 0, right: 0, bottom: 2, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 4, top: 0, right: 1, bottom: 3, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 5, top: 0, right: 1, bottom: 4, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 6, top: 0, right: 1, bottom: 5, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 7, top: 0, right: 1, bottom: 5, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 8, top: 0, right: 2, bottom: 3, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 9, top: 0, right: 2, bottom: 4, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 10, top: 0, right: 2, bottom: 4, left: 2, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 11, top: 0, right: 2, bottom: 5, left: 1, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 12, top: 3, right: 3, bottom: 4, left: 4, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 13, top: 3, right: 3, bottom: 5, left: 5, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 14, top: 3, right: 4, bottom: 5, left: 4, place: Position::Unknown, rotation:Rotation::NoTurn},
            Piece {number: 15, top: 3, right: 5, bottom: 5, left: 5, place: Position::Unknown, rotation:Rotation::NoTurn},
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
