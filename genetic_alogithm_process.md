# Genetic algorithm process description

## First step : fetching the pieces file

- Get the pieces file into memory
- Put the file's data into a struct (dimensions, list of pieces with sides, position and rotation)

## Second step : initializing the puzzle

- Build a puzzle from the file's data where the corners pieces are in the corners, etc...

## Third step : initializing the starting population

- Load the puzzle's characteristics
- Create an array or a vector with n initialized puzzles
- Compute each individual's fitness (and store it in an array)

## Fourth step : performing the algorithm

- Select the best individuals from the population
- Cross them between each other
- Perform random mutations on the new indiviuals
- Select the best and make them the new population
- *Optional* | Add the 10% best individuals from the previous population
- Compute each individual's fitness from the new population (and store it in an array)
- Compute the new population's average, min and max score

## Final step : getting the results

- Print the final population's features
- Print the best individual's features
- Display a graph of the evolution of the scores (avg, min, max)
