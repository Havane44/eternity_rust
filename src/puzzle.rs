use std::fs;
use rand::Rng;

#[derive(PartialEq, Clone)]
/// Models the possible rotation that can be performed on a puzzle piece.
pub enum Rotation {
    Clockwise,
    CounterClockwise
}

/// Models the possible states a piece can be in.
#[derive(PartialEq, Clone)]
pub enum RotationState {
    NoRotation,
    QuarterTurn,
    HalfTurn,
    ThreeQuartersTurn
}

/// Updates the rotation state of a puzzle piece.
pub fn update_angular_state(state: RotationState, rotation: Rotation)-> RotationState {
    match rotation {
        Rotation::Clockwise => {
            match state {
                RotationState::NoRotation => RotationState::QuarterTurn,
                RotationState::QuarterTurn => RotationState::HalfTurn,
                RotationState::HalfTurn => RotationState::ThreeQuartersTurn,
                RotationState::ThreeQuartersTurn => RotationState::NoRotation
            }
        },
        Rotation::CounterClockwise => {
            match state {
                RotationState::NoRotation => RotationState::ThreeQuartersTurn,
                RotationState::QuarterTurn => RotationState::NoRotation,
                RotationState::HalfTurn => RotationState::QuarterTurn,
                RotationState::ThreeQuartersTurn => RotationState::HalfTurn
            }
        }
    }
}


#[derive(PartialEq, Clone, Debug)]
/// Possible positions of a puzzle piece on the board.
pub enum Position {
    Corner,
    Border,
    Middle
}

#[derive(PartialEq, Clone)]
/// Models a puzzle piece.
pub struct Piece {
    sides: [u32; 4],
    pub place: Position,
    pub angular_position: RotationState
}

impl Piece {
    /// Rotates the puzzle piece.
    pub fn rotate(mut self, rotation: Rotation) {
        match rotation {
            Rotation::Clockwise => {
                let temp = self.sides[0];
                for i in 0..(self.sides.len()-2) {
                    self.sides[i] = self.sides[i+1];
                }
                self.sides[3] = temp;
                self.angular_position = update_angular_state(self.angular_position, rotation)
            },
            Rotation::CounterClockwise => {
                let temp = self.sides[3];
                for i in 1..(self.sides.len()-1) {
                    self.sides[i] = self.sides[i-1];
                }
                self.sides[0] = temp;
                self.angular_position = update_angular_state(self.angular_position, rotation)
            }
        }
    }
}

/// Determines the position of a piece, given its characteristics.
pub fn determine_position(sides: &[u32; 4]) -> Position {
    let mut count: u8 = 0;
    for i in *sides {
        if i == 0 {
            count += 1;
        }
    }
        
    return match count {
        0 => Position::Middle, 
        1 => Position::Border, 
        2 => Position::Corner,
        _=> panic!()
    };
}

/// Puzzle characteristics, to be extracted from a pieces' set file.
pub struct Puzzle {
    height: u32,
    width: u32,
    pieces: Vec<Piece>,
}

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
            let sides: [u32; 4] = [sides[0], sides[1], sides[2], sides[3]];
            pieces_list.push(Piece { sides: sides, place: determine_position(&sides), angular_position: RotationState::NoRotation });
        }
        
        return Puzzle {height: dimensions[0], width: dimensions[1], pieces: pieces_list};
    }

    /// Prints the puzzle's characteristics.
    pub fn show(&self) {
        println!("Dimensions : {} x {}", self.height, self.width);
        println!();

        println!("Liste des pièces (dans l'ordre) :");
        for piece in &self.pieces {
            println!("{} {} {} {}", piece.sides[0], piece.sides[1], piece.sides[2], piece.sides[3]);
        }

        println!();
    }

    pub fn max_fitness(&self) -> u32 {
        2*(self.height-1)*self.width
    }
}

/// Sorts the puzzles pieces and return three vectors, each containing only one piece type.
pub fn sort_pieces(puzzle: &Puzzle) -> (Vec<Piece>, Vec<Piece>, Vec<Piece>) {
    let mut corners: Vec<Piece> = Vec::new();
    let mut borders: Vec<Piece> = Vec::new();
    let mut middles: Vec<Piece> = Vec::new();

    for piece in &puzzle.pieces {
        match piece.place {
            Position::Corner => corners.push(piece.clone()),
            Position::Border => borders.push(piece.clone()),
            Position::Middle => middles.push(piece.clone()),
        }
    }

    return (corners, borders, middles);

}

#[derive(PartialEq, Clone)]
/// Puzzle board.
pub struct PuzzleBoard {
    height: u32,
    width: u32,
    pub board: Vec<Vec<Piece>>
}

impl PuzzleBoard {
    /// Make a puzzle by randomly putting the pieces (at the right places: corners on the corners, etc...).
    pub fn init(puzzle: &Puzzle, original_corners: &Vec<Piece>, original_borders: &Vec<Piece>, original_middles: &Vec<Piece>) -> PuzzleBoard{

        let mut corners = original_corners.clone();
        let mut borders = original_borders.clone();
        let mut middles = original_middles.clone();

        let mut board: Vec<Vec<Piece>> = Vec::<Vec<Piece>>::new();

        for i in 0..puzzle.height {
            board.push(Vec::<Piece>::new());
            for j in 0..puzzle.width {
                if i == 0 || i == puzzle.height - 1 {
                    if j == 0 || j == puzzle.width - 1 {
                        // Corner
                        if corners.len() == 0 {
                            continue;
                        } else {
                            let piece_index = rand::thread_rng().gen_range(0..corners.len());
                            board[i as usize].push(corners[piece_index].clone());
                            corners.remove(piece_index);
                        }
                    } else {
                        // Top or bottom border
                        if borders.len() == 0 {
                            continue;
                        } else {
                            let piece_index = rand::thread_rng().gen_range(0..borders.len());
                            board[i as usize].push(borders[piece_index].clone());
                            borders.remove(piece_index);
                        }
                    }
                } else if  j == 0 || j == puzzle.width - 1 {
                    if borders.len() == 0 {
                        continue;
                    } else {
                        let piece_index = rand::thread_rng().gen_range(0..borders.len());
                        board[i as usize].push(borders[piece_index].clone());
                        borders.remove(piece_index);
                    }
                } else {
                    // Middle
                    if middles.len() == 0 {
                        continue;
                    } else {
                        let piece_index = rand::thread_rng().gen_range(0..middles.len());
                        board[i as usize].push(middles[piece_index].clone());
                        middles.remove(piece_index);
                    }
                }
            }
        }

        return PuzzleBoard {height: puzzle.height, width: puzzle.width, board: board}
    }

    /// Perform mutations on a puzzle.
    pub fn mutate() {
        todo!();
    }

    /// Compute the number of matching edges inside the puzzle.
    pub fn fitness(&self) -> u32 {
        let mut score: u32 = 0;

        // Vertically
        for i in 0..((self.height) as usize) {
            for j in 0..((self.width - 1) as usize) {
                if self.board[i][j].sides[1] == self.board[i][j+1].sides[3] {
                    score += 1;
                }
            }
        }

        // Horizontally
        for i in 0..((self.height - 1) as usize) {
            for j in 0..(self.width as usize) {
                if self.board[i][j].sides[2] == self.board[i+1][j].sides[0] {
                    score += 1;
                }
            }
        }

        return score;
    }

    /// Compute the maximal fitness that the board can have, based on the fitness() method
    pub fn max_fitness(&self) -> u32 {
        2*(self.height-1)*self.width
    }

    /// Compute the number of 2x2 regions that match in their inner adjacent inner sides
    pub fn fitness_2(&self) -> u32 { todo!(); }

    /// Compute the maximal fitness that the board can have, based on the fitness_2() method
    pub fn max_fitness_2(&self) -> u32 {
        todo!()
    }

    /// Compute the number of tiles that have their 4 edges matching with adjacent tiles
    pub fn fitness_3(&self) -> u32 { todo!(); }

    /// Compute the maximal fitness that the board can have, based on the fitness_3() method
    pub fn max_fitness_3(&self) -> u32 {
        todo!()
    }

    /// Compute the number of 3x3 regions that match in their inner adjacent inner sides
    pub fn fitness_4(&self) -> u32 { todo!(); }

    /// Compute the maximal fitness that the board can have, based on the fitness_4() method
    pub fn max_fitness_4(&self) -> u32 {
        todo!()
    }

    /// Prints the content of the puzzle to the standard output.
    pub fn show(&self) {
        println!("Dimensions : {} x {}", self.height, self.width);
        println!();

        for i in 0..(&self.board.len() - 1) {
            let mut count = 0;
            for _piece in &self.board[i] {
                count = count + 1;
            }
            println!("Number of pieces on line {}: {}", i, count);
        }
    }

}
