pub mod puzzle;

use core::panic;
use std::env;

use crate::puzzle::*;

//TODO: Function to select the individuals of a population
fn random_selection() {
    todo!();
}

//TODO: Function to cross two puzzles
fn crossing(puzzle1: Puzzle, puzzle2: Puzzle) -> (Puzzle, Puzzle) {
    todo!();
}

//TODO: Function to perform random mutations on a given puzzle
fn mutation() {
    todo!();
}


fn main() {
    // Récupération du chemin du fichier des pièces
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path.as_str(), 
        None => panic!("No arguments passed !"),
    };

    // Paramètres de l'algorithme
    const SIZE: usize = 100;
    const ITERATIONS: usize = 100;

    // Initialisation de la population de départ

    let mut population: Vec<Puzzle> = Vec::new();
    for i in 0..SIZE {
        population.push(Puzzle::new(file_path));
        population[i].init();
    }

    let scores: Vec<u32> = population.iter().map(|puzzle| puzzle.fitness()).collect();

    // Boucle principale de l'algorithme
    for i in 0..ITERATIONS {

        println!("Itération n°{}", i);

        //TODO: Sélection des individus pour former la nouvelle population

        //TODO: Mutations aléatoires

        //TODO: Remplacement

        //TODO: Evaluation des scores des puzzles de la nouvelle population
        
    }
    

}
