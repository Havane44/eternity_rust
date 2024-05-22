use rayon::prelude::*;

use crate::puzzle::*;

pub struct Population {
    pub individuals: Vec<PuzzleBoard>,
    pub scores: Vec<u32>
}

impl Population {
    /// Creates a new population of size n from a Puzzle struct.
    pub fn new(n: &u32, puzzle: &Puzzle) -> Population {

        let (corners, borders, middles) = sort_pieces(&puzzle);

        let mut individuals: Vec<PuzzleBoard> = Vec::new();
        for _i in 0..*n {
            individuals.push(PuzzleBoard::init(&puzzle, &corners, &borders, &middles));
        }

        let scores = individuals.par_iter().map(|puzzle| puzzle.fitness()).collect();

        return Population { 
            individuals: individuals, 
            scores: scores
        }
    }

    /// Update the scores Vec.
    pub fn update_scores(mut self) {
        self.scores = self.individuals.par_iter().map(|puzzle| puzzle.fitness()).collect();
    }

    ///TODO: Select the individuals of a population for later crossings
    pub fn random_selection(_population: &Vec<Puzzle>) -> Vec<usize> {
        todo!();
    }

    ///TODO: Crosses two puzzles
    pub fn crossing(_puzzle1: &Puzzle, _puzzle2: &Puzzle) -> (Puzzle, Puzzle) {
        todo!();
    }

    ///TODO: Perform mutations on the individuals
    pub fn mutation() {
        todo!();
    }
}