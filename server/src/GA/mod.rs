pub mod functions;
pub mod types;
pub mod routes;

use std::{cmp::Ordering};
use rand::Rng;
use types::*;

fn compare_population(
    first: &Genome,
    second: &Genome,
    fitness_func: fn(genome: Genome, max_warmness: f32) -> f32,
    max_warmness: f32
) -> Ordering {
    if fitness_func(first.clone(), max_warmness) > fitness_func(second.clone(), max_warmness) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

pub fn accuracy(genome: Genome, target: f32, fitness_func: FitnessFunc) -> f32 {
    let fit_value = fitness_func(genome, target);
    100.0 * fit_value / target
}

pub fn run_evolution(
    items: GeneList,
    population_size: i32,
    generation_limit: i32,
    fitness_limit: f32,
    mutation_number: i32,
    mutation_prob: f32,
    fitness_func: FitnessFunc,
    generate_population: fn(i32, GeneList) -> Population,
    selection_func: fn(population: Population, fitness_func: FitnessFunc, fitness_limit: f32) -> (Genome, Genome),
    mutation_func: fn(genome: Genome, num: i32, probability: f32, GeneList) -> Genome,
    crossover_func: fn(a: Genome, b: Genome) -> (Genome, Genome)
) -> Population {
    let mut population = generate_population(population_size, items.clone());

    for i in 0..generation_limit {
        //sort population (reverse)
        population.sort_by(|a, b| compare_population(a, b, fitness_func, fitness_limit));
        //check fitness of grater fit
        if fitness_func(population[0].clone(), fitness_limit) >= fitness_limit {
            break;
        }

        let mut next_generation = population[0..2].to_vec();
        
        //crossover
        for j in 0..(population.len() / 2) {
            let parents = selection_func(population.clone(), fitness_func, fitness_limit);
            let (mut offspring_a, mut offspring_b) = crossover_func(parents.0, parents.1);

            //mutation
            offspring_a = mutation_func(offspring_a, mutation_number, mutation_prob, items.clone());
            offspring_b = mutation_func(offspring_b, mutation_number, mutation_prob, items.clone());

            //next generation
            next_generation.push(offspring_a);
            next_generation.push(offspring_b);
        }

        population = next_generation;
    }

    //sort result
    population.sort_by(|a, b| compare_population(a, b, fitness_func, fitness_limit));
    //return population
    population
}


