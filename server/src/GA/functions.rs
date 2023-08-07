use std::cmp::Ordering;

use rand::Rng;
use super::types::*;
use random_choice::*;
use colors_transform::*;

pub fn single_pair_crossover(mut a: Genome, mut b: Genome) -> (Genome, Genome) {
    let length = a.len();
    if length < 2 {
        return (a, b)
    }

    let mut rnd = rand::thread_rng();
    let index = rnd.gen_range(1..length); 

    a[0..index].to_vec().append(&mut b[0..index].to_vec());
    b[0..index].to_vec().append(&mut a[0..index].to_vec());

    (a, b)

}

pub fn mutation(mut genome: Genome, num: i32, probability: f32, genes: GeneList) -> Genome {
    let mut rnd = rand::thread_rng();
    for i in 0..num {
        let index = rnd.gen_range(0..genome.len());
        if rnd.gen::<f32>() <= probability {
            let random_genome = generate_genome(genes.clone());
            genome[index] = random_genome[index].clone();
        }
    }
    genome

}

pub fn selection_pair(population: Population, fitness_func: FitnessFunc, max_warmness: f32) -> (Genome, Genome) {
    let mut weights: Vec<f32> = Vec::new();
    for genome in population.iter() {
        weights.push(fitness_func(genome.to_vec(), max_warmness));
    }
    let results = random_choice().random_choice_f32(&population, &weights, 2);
    (results[0].to_vec(), results[1].to_vec())
}

pub fn warmness_fitness(genome: &Genome, max_warmness: f32) -> f32 {
    let mut warmness: f32 = 0.0;
    for gene in genome.iter() {
        warmness += gene.warmness;
        if warmness > max_warmness {
            return 0.0;
        }
    }
    warmness
}

pub fn fitness(genome: Genome, max_warmness: f32) -> f32 {
    let warmness = warmness_fitness(&genome, max_warmness);
    let color_fitness = colorFitness(&genome, 5);

    warmness / (1.0 + color_fitness / 180.0)
}

pub fn colorFitness(genome: &Genome, d_error: i32) -> f32 {
    let mut color_array: Vec<f32> = Vec::new();
    for gene in genome.iter() {
        let hue = Rgb::from_hex_str(&gene.color1).unwrap().get_hue();
        color_array.push(hue);
    }
    let colors = findColors(color_array, 10);
    let mut color_avg = averageColor(colors);
    color_avg.sort_by(|a, b| if a < b { Ordering::Less } else { Ordering::Greater });
    let distances = compute_distances(color_avg.clone(), d_error);
    if color_avg.len() == 1 {
        return 0.0;
    }

    let mut total: f32 = 0.0;
    if color_avg.len() > 2 {
        total = color_avg.last().unwrap() - color_avg.first().unwrap();
    } else if color_avg.len() == 2 {
        total = 180.0;
    } else {
        total = d_error as f32;
    }

    let target = total / distances.len() as f32;
    let d_distances: Vec<f32> = distances.iter().map(|d| (target - d).abs()).collect();
    d_distances.iter().sum::<f32>() / d_distances.len() as f32
}

#[derive(Debug)]
struct ColorGroup {
    id: f32,
    colors: Vec<f32>
}

fn compute_distances(color_array: Vec<f32>, d_error: i32) -> Vec<f32> {
    let mut distances: Vec<f32> = Vec::new();
    for i in 0..color_array.len() - 1 {
        let distance = (color_array[i] - color_array[i+1]).abs();
        distances.push(distance);
    }
    distances
}

fn averageColor(color_groups_array: Vec<ColorGroup>) -> Vec<f32> {
    let mut color_averages: Vec<f32> = Vec::new();
    for color in color_groups_array.iter() {
        let avg: f32 = color.colors.iter().sum::<f32>() / color.colors.len() as f32;
        color_averages.push(avg);
    }

    color_averages
}

fn findColors(color_array: Vec<f32>, d_error: i32) -> Vec<ColorGroup> {
    let mut colors: Vec<ColorGroup> = Vec::new();
    for color in color_array.iter() {
        if colors.len() == 0 {
            colors.push( ColorGroup { id: *color, colors: vec![*color] } )
        } else {
            let index = colors.iter()
                .position(|c| (c.id - color).abs() <= d_error as f32 || (c.id + color).abs() <= d_error as f32);
            match index {
                Some(i) => colors[i].colors.push(*color),
                None => colors.push( ColorGroup {id: *color, colors: vec![*color]})
            }
        }
    }
    colors
}


pub fn generate_population(size: i32, genes: GeneList) -> Population {
    let mut list: Population = Vec::new();
    let mut rnd = rand::thread_rng();

    for i in 0..size {
        let genome = generate_genome(genes.clone());
        list.push(genome)
    }

    list
}

//for each category takes a random item
pub fn generate_genome(genes: GeneList) -> Genome {
    let mut rnd = rand::thread_rng();
    let mut genome: Genome = Vec::new();
    let hat =        genes.hat[rnd.gen_range(0..genes.hat.len())].clone();
    let shirt =       genes.shirt[rnd.gen_range(0..genes.shirt.len())].clone();
    let sweater =     genes.sweater[rnd.gen_range(0..genes.sweater.len())].clone(); 
    let jacket =     genes.jacket[rnd.gen_range(0..genes.jacket.len())].clone(); 
    let trousers =   genes.trousers[rnd.gen_range(0..genes.trousers.len())].clone();
    let socks =      genes.socks[rnd.gen_range(0..genes.socks.len())].clone(); 
    let shoes =      genes.shoes[rnd.gen_range(0..genes.shoes.len())].clone();

    genome.push(hat);
    genome.push(shirt);
    genome.push(sweater);
    genome.push(jacket);
    genome.push(trousers);
    genome.push(socks);
    genome.push(shoes);
    genome
}
