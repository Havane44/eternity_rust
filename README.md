# FRENCH - FRANÇAIS

**Auteur : Antoine JONCHERAY**

Implémentation d'un algorithme génétique pour résoudre le puzzle "Eternity", écrit en Rust.

Fichiers : 
- `main.rs` : Coeur de l'algorithme génétique.
- `puzzle.rs` : Structures et fonctions utiles pour modéliser et manipuler le puzzle.

Bibliographie : 


# ENGLISH - ANGLAIS

*Work in progress...*

# To-do list

- [x] compute the maximum score a puzzle can have based on the dimensions (fn maximum_score(n) -> u32)
- [x] start genetic algorithm scaffolding (put the functions in order and start designing the loop, the populations, etc...)
- [x] update the `Puzzle` and `Piece` structs
- [x] Write the `PuzzleBoard.init()` function
- [ ] implement selection functions (random, tournament)
- [ ] implement crossing function
- [ ] implement mutation function
- [ ] implement replacement function (all the population)


# Elements

- `main.rs`
- `puzzle.rs` : 
    - Rotation
    - RotationState
        - update_angular_state
    - Position
    - Piece
        - rotate
        - determine_position
    - Puzzle
        - new
        - show
        - sort_pieces
    - PuzzleBoard
        - init
        - fitness
        - max_fitness
        - show 
- `population.rs` : 
    - Population
        - new
        - update_scores
        - random_selection
        - crossing
        - mutation