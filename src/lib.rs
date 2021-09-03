//! `boids` provides a simple library for creating and managing a flock of boids.

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

pub mod boids;
pub use boids::boid2d::Boid2D;
pub mod flock;
pub use flock::Flock;
