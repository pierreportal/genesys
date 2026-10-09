pub trait Genome: Clone {
    fn random() -> Self;
    fn mutate(&mut self);
    fn crossover(&self, partner: &Self) -> Self;
    fn derive(population_size: usize) -> Vec<Self> {
        (0..population_size).map(|_| Self::random()).collect()
    }
}
