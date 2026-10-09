use rand::Rng;
use std::fmt::Debug;

use crate::fitness::Fitness;
use crate::genome::Genome;

const DEFAULT_SURVIVOR_RATIO: f32 = 0.1;
const DEFAULT_MUTATION_RATE: f64 = 0.2;

pub trait Selection<G> {
    fn select(&mut self) -> Vec<G>;
}

#[derive(Debug)]
pub struct EvolutionResult<'a, G> {
    pub best: &'a G,
    pub generations: usize,
    pub solved: bool,
}

pub struct Genesys<G>
where
    G: Genome + Fitness + Debug,
{
    pub population: Vec<G>,
    pub survivor_ratio: f32,
    pub mutation_rate: f64,
}

impl<G> Genesys<G>
where
    G: Genome + Fitness + Debug,
{
    pub fn new(population: Vec<G>) -> Self {
        Self {
            population,
            survivor_ratio: DEFAULT_SURVIVOR_RATIO,
            mutation_rate: DEFAULT_MUTATION_RATE,
        }
    }

    pub fn survivor_ratio(&mut self, survivor_ratio: f32) -> &mut Self {
        self.survivor_ratio = survivor_ratio;
        self
    }

    pub fn mutation_rate(&mut self, mutation_rate: f64) -> &mut Self {
        self.mutation_rate = mutation_rate;
        self
    }

    fn evolve_population(&mut self) {
        let survivors = self.select();

        let mut next_generation = survivors.clone();

        let mut rng = rand::thread_rng();

        while next_generation.len() < self.population.len() {
            let parent_a = &survivors[rng.gen_range(0..survivors.len())];
            let parent_b = &survivors[rng.gen_range(0..survivors.len())];
            let mut child = parent_a.crossover(parent_b);

            if rng.gen_bool(self.mutation_rate) {
                child.mutate();
            }
            next_generation.push(child);
        }
        self.population = next_generation;
    }

    pub fn evolve_until<F>(&mut self, generations: usize, condition: F) -> EvolutionResult<'_, G>
    where
        F: Fn(&G) -> bool,
    {
        let mut solved = false;
        let mut generation = 0;

        for _ in 0..generations {
            if condition(self.best()) {
                solved = true;
                break;
            }
            generation += 1;
            self.evolve_population();
        }

        EvolutionResult {
            best: self.best(),
            generations: generation,
            solved,
        }
    }
    pub fn evolve(&mut self, generations: usize) -> &mut Self {
        for _generation in 0..generations {
            self.evolve_population();
        }
        self
    }

    pub fn best(&self) -> &G {
        &self.population[0]
    }
}

impl<G> Selection<G> for Genesys<G>
where
    G: Genome + Fitness + Debug,
{
    fn select(&mut self) -> Vec<G> {
        self.population
            .sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());

        let survivor_count =
            ((self.population.len() as f32 * self.survivor_ratio).round() as usize).max(2);

        self.population[..survivor_count].to_vec()
    }
}
