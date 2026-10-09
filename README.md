# Genesys

A small, dependency-light genetic algorithm framework for Rust.

Genesys lets you evolve any type that knows how to generate itself, mutate, and
crossover. You only describe the genome and how to score it — the framework
handles selection, breeding, and iterating generation after generation.

## Features

- **Simple traits** — implement `Genome` and `Fitness` and you're done.
- **Builder-style configuration** — chain `survivor_ratio` and `mutation_rate`.
- **Two evolution modes** — run a fixed number of generations, or evolve until a
  goal condition is met.
- **Pluggable selection** — the `Selection` trait controls which genomes survive.

## Project layout

```
genesys/
├── src/main.rs        # Example binary: evolves a 2D vector toward the origin
└── genesys-rs/        # The library crate
    └── src/
        ├── genome/    # Genome trait
        ├── fitness/   # Fitness trait
        └── genesys/   # Genesys engine + Selection trait
```

The workspace root is a runnable demo that depends on the `genesys-rs` library.

## Getting started

### As a dependency

Add the library to your `Cargo.toml`. While `genesys-rs` is not published yet you
can reference it by path, or clone it as a path dependency:

```toml
[dependencies]
genesys-rs = { path = "./genesys-rs" }
rand = "0.8"
```

### Defining a genome

A genome must be `Clone` and implement `random`, `mutate`, and `crossover`. The
trait also provides a `derive(population_size)` helper that builds the initial
population for you.

```rust
use genesys_rs::Genome;
use rand::Rng;

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
            Vector { x: self.x, y: partner.y }
        } else {
            Vector { x: partner.x, y: self.y }
        }
    }
}
```

### Scoring a genome

Lower values are treated as better fitness.

```rust
use genesys_rs::Fitness;

impl Fitness for Vector {
    fn fitness(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}
```

### Running the evolution

```rust
use genesys_rs::Genesys;

fn main() {
    let population = Genesys::new(Vector::derive(100));

    let goal = |genome: &Vector| genome.fitness() < 0.0001;

    let result = population
        .survivor_ratio(0.1)
        .mutation_rate(0.2)
        .evolve_until(1000, goal);

    println!("solved: {}", result.solved);
    println!("generations: {}", result.generations);
    println!("best: {:#?}", result.best);
}
```

## API overview

### `Genome`

| Method | Purpose |
| --- | --- |
| `random()` | Create a random initial genome. |
| `mutate(&mut self)` | Introduce a random change in place. |
| `crossover(&self, partner: &Self) -> Self` | Produce a child from two parents. |
| `derive(population_size: usize) -> Vec<Self>` | Build an initial population (provided). |

### `Fitness`

| Method | Purpose |
| --- | --- |
| `fitness(&self) -> f32` | Score the genome; lower is better. |

### `Genesys<G>`

| Method | Purpose |
| --- | --- |
| `new(population: Vec<G>) -> Self` | Start from an existing population. |
| `survivor_ratio(f32) -> &mut Self` | Fraction of each generation that survives (default `0.1`). |
| `mutation_rate(f64) -> &mut Self` | Probability a child mutates (default `0.2`). |
| `evolve(generations: usize) -> &mut Self` | Run a fixed number of generations. |
| `evolve_until(generations, condition) -> EvolutionResult` | Run until a goal is met or the limit is reached. |
| `best() -> &G` | The highest-fitness genome in the current population. |

### `EvolutionResult`

| Field | Description |
| --- | --- |
| `best: &G` | Best genome found. |
| `generations: usize` | Generations elapsed before stopping. |
| `solved: bool` | Whether the goal condition was satisfied. |

### `Selection`

Implement `Selection<G>` to customize how survivors are chosen. The default
implementation sorts the population by fitness and keeps the top
`survivor_ratio` fraction (at least two).

## Building and running

```bash
# Run the bundled example
cargo run

# Build the library only
cargo build -p genesys-rs
```

Requires Rust with the 2024 edition.

## Notes

- `best()` assumes the population is sorted, which happens during selection.
- `fitness()` must not return `NaN`; comparisons rely on `partial_cmp`.

## License

No license has been specified yet. Add one before distributing or publishing.
