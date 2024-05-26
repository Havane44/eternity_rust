use rand::thread_rng;
use rayon::prelude::*;
use rand::seq::SliceRandom;

use crate::puzzle::*;

/// Models a population of individuals (here, puzzle boards).
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

    /// Select some individuals of a population according to the "roulette" method.
    pub fn roulette_selection(&self, amount: usize) -> Vec<usize> {
        let mut res:Vec<usize> = Vec::new();
        let mut indexes_with_frequencies: Vec<usize> = Vec::new();
        for i in 0..self.scores.len() {
            for _j in 0..i {
                indexes_with_frequencies.push(i);
            }
        }

        for _i in 0..amount {
            res.push(*indexes_with_frequencies.choose(&mut thread_rng()).unwrap());
        }

        return res;
    }

    /// Select some individuals of a population according to the tournament method
    pub fn tournament_selection(&self, amount: usize) -> Vec<usize> {

        // FIXME

        let mut res: Vec<usize> = Vec::new();

        for _i in 0..amount {
            // Pick two random individuals
            let score1 = self.scores.choose(&mut thread_rng()).unwrap();
            let score1_index = self.scores.iter().position(|x| x == score1).unwrap();
            let score2 = self.scores.choose(&mut thread_rng()).unwrap();
            let score2_index = self.scores.iter().position(|x| x == score1).unwrap();
            // Only append the one with the highest score
            if score1 > score2 {
                res.push(score1_index);
            } else {
                res.push(score2_index);
            }
        }
        return res;
    }
}

/// Crosses two puzzles. Returns two new puzzles
pub fn crossing() -> () {
    todo!();
}