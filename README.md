# FRENCH - FRANÇAIS

**Auteur : Antoine JONCHERAY**

Implémentation d'un algorithme génétique pour résoudre le puzzle "Eternity", écrit en Rust.

Fichiers : 
- `main.rs` : Coeur de l'algorithme génétique.
- `puzzle.rs` : Structures et fonctions pour modéliser et manipuler le puzzle.
- `population.rs` : Structure et fonctions pour manipuler une population d'individus (ici des puzzles)

Bibliographie : 


# ENGLISH - ANGLAIS

*Work in progress...*

# To-do list

- [x] compute the maximum score a puzzle can have based on the dimensions (fn maximum_score(n) -> u32)
- [x] start genetic algorithm scaffolding (put the functions in order and start designing the loop, the populations, etc...)
- [x] update the `Puzzle` and `Piece` structs
- [x] Write the `PuzzleBoard.init()` function
- [x] Implement selection function (roulette)
- [x] implement selection function (tournament) (FIXME)
- [ ] design and implement crossing function
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
    - mutate
    - fitness
    - max_fitness
    - fitness_2
    - max_fitness_2
    - fitness_3
    - max_fitness_3
    - fitness_4
    - max_fitness_4
    - show 
- `population.rs` : 
  - Population
    - new
    - update_scores
    - random_selection
    - crossing
    - mutation