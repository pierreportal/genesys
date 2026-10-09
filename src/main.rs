use genesys_rs::{Fitness, Genesys, Genome};
use rand::Rng;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Vector {
    x: f32,
    y: f32,
}

impl Genome for Vector {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(-10.0..10.0),
            y: rng.gen_range(-10.0..10.0),
        }
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        self.x += rng.gen_range(-0.5..0.5);
        self.y += rng.gen_range(-0.5..0.5);
    }

    fn crossover(&self, partner: &Self) -> Self {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            Vector {
                x: self.x,
                y: partner.y,
            }
        } else {
            Vector {
                x: partner.x,
                y: self.y,
            }
        }
    }
}

impl Fitness for Vector {
    fn fitness(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

fn main() {
    let p_size = 100;
    let generations = 1000;
    let fitness_threshold = 0.0001;

    let pool = Vector::derive(p_size);

    let mut population = Genesys::new(pool);

    let goal = |genome: &Vector| genome.fitness() < fitness_threshold;

    let best_candidate = population
        .survivor_ratio(0.1)
        .mutation_rate(0.2)
        .evolve_until(generations, goal);

    println!("{:#?}", best_candidate);
}
