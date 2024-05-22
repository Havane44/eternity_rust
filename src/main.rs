pub mod puzzle;
pub mod population;

use core::panic;
use std::env;
use rayon::prelude::*;

use crate::puzzle::*;
use crate::population::*;

fn main() {
    // Récupération du chemin du fichier des pièces
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path.as_str(), 
        None => panic!("No arguments passed !"),
    };

    let puzzle_characteristics = Puzzle::new(file_path);

    // Paramètres de l'algorithme
    const SIZE: u32 = 50;
    const ITERATIONS: usize = 10;

    // Initialisation de la population de départ (individus + scores)

    println!("");
    println!("------------------------");
    println!("Initialisation de la population");
    println!("------------------------");
    println!("");

    let population: Population = Population::new(&SIZE, &puzzle_characteristics);

    // Initialisation des stats
    let mut max_score: Vec<u32> = Vec::new();
    let mut min_score: Vec<u32> = Vec::new();
    let mut avg_score: Vec<f32> = Vec::new();

    max_score.push(*population.scores.par_iter().max().unwrap());
    min_score.push(*population.scores.par_iter().min().unwrap());
    avg_score.push(((population.scores.iter().sum::<u32>()) / (population.scores.len() as u32)) as f32);

    println!("Nombre de puzzles de la population : {}", population.individuals.len());
    println!("Meilleur score : {}", &max_score[0]);
    println!("Pire score : {}", &min_score[0]);
    println!("Score moyen : {}", &avg_score[0]);
    println!("Score des individus : ");
    for score in &population.scores {
        print!("{} ", score);
    }
    println!("");
    println!("");

    // Boucle principale de l'algorithme
    println!("------------------------");
    println!("Début de l'algorithme");
    println!("------------------------");
    println!("");

    for _i in 0..ITERATIONS {

        //TODO: Sélection des individus pour former la nouvelle population

        //TODO: Croisements
        
        //TODO: Création de la nouvelle population

        //TODO: Mutations aléatoires

        //TODO: Remplacement

        //TODO: Evaluation de la population

        // scores = population.individuals.par_iter().map(|puzzle| puzzle.fitness()).collect();

        // max_score.push(*scores.par_iter().max().unwrap());
        // min_score.push(*scores.par_iter().min().unwrap());
        // avg_score.push(((scores.iter().sum::<u32>()) / (scores.len() as u32)) as f32);

    }

    println!("");
    println!("------------------------");
    println!("Résulat de l'algorithme");
    println!("------------------------");
    println!("");

}
