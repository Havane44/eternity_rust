pub mod puzzle;

use core::panic;
use std::env;

use rayon::prelude::*;

use crate::puzzle::*;

//TODO: Function to select the individuals of a population
fn random_selection(population: &Vec<Puzzle>) -> Vec<usize> {
    todo!();
}

//TODO: Function to cross two puzzles
fn crossing(puzzle1: &Puzzle, puzzle2: &Puzzle) -> (Puzzle, Puzzle) {
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
    for _i in 0..SIZE {
        population.push(Puzzle::new(file_path));
    }

    let mut max_score: Vec<u32> = Vec::new();
    let mut min_score: Vec<u32> = Vec::new();
    let mut avg_score: Vec<f32> = Vec::new();

    // Boucle principale de l'algorithme
    for i in 0..ITERATIONS {

        println!("Itération n°{}", i);

        // Evaluation de la population
        let mut scores: Vec<u32> = population.par_iter().map(|puzzle| puzzle.fitness()).collect();

        max_score.push(*scores.par_iter().max().unwrap());
        min_score.push(*scores.par_iter().min().unwrap());
        avg_score.push(((scores.iter().sum::<u32>()) / (scores.len() as u32)) as f32);


        //TODO: Sélection des individus pour former la nouvelle population
        let new_population_indexes = random_selection(&population);

        //TODO: Croisements
        let mut crossing_results: Vec<Puzzle> = Vec::new();
        for i in new_population_indexes.iter() {
            for j in new_population_indexes.iter() {
                if i != j {
                    let crossings = crossing(&population[*i], &population[*j]);
                    crossing_results.push(crossings.0);
                    crossing_results.push(crossings.1);
                }
            }
        }
        //TODO: Création de la nouvelle population
        let mut new_population: Vec<Puzzle> = Vec::new();

        //TODO: Mutations aléatoires

        //TODO: Remplacement
        population = new_population;

    }
    

}
