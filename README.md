# Exhaustive Search for TSP

This repo contains a Rust implementation of a simple exhaustive search for TSP instances.
The search can be parallelized using [rayon](https://crates.io/crates/rayon), but was observed to extend runtime.

# Running the code

1. Get the Rust compiler / `cargo` from here: https://www.rust-lang.org/learn/get-started
2. Run the program
```Bash
	$ cargo run --release 
```
