# boids
[![Crates.io](https://img.shields.io/crates/v/boids)](https://crates.io/crates/boids) 
[![Docs.rs](https://docs.rs/boids/badge.svg)](https://docs.rs/boids) 
[![Build](https://github.com/Ewpratten/boids/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/boids/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/boids/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/boids/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/boids/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/boids/actions/workflows/audit.yml)


repo description

## Parallel processing & speed

When compiled with the `rayon` feature enabled, the library will use the `rayon` crate to parallelize the computation. This may or may not be something you want to do, depending on your application.

| Boid count | Single-thread Time | Parallel Time |
|------------|--------------------|---------------|
| 100        | 7.3ms              | 3.5ms         |
| 1000       | 414ms              | 42.5ms        |
| 10000      | ???                | 3.4s          |

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install boids
```

```
libgtk-3-dev libatk1.0-dev libsdl-pango-dev libcairo2-dev
``` 