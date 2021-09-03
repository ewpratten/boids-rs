use cgmath::{num_traits::Float, BaseNum, Vector3};

use crate::flock::Flock;

pub mod boid2d;
pub mod boid3d;
mod convert;
mod limits;

/// Defines the force weights for a boid
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoidWeights<U: BaseNum> {
    pub alignment: U,
    pub cohesion: U,
    pub separation: U,
    pub targeting: U,
}

impl<U: BaseNum + Float> Default for BoidWeights<U> {
    fn default() -> Self {
        Self {
            alignment: U::from(1.5).unwrap(),
            cohesion: U::from(1.0).unwrap(),
            separation: U::from(1.0).unwrap(),
            targeting: U::from(0.0003).unwrap(),
        }
    }
}

/// Common code across all boids no matter their dimensions
pub trait Boid<T: Boid<T, U>, U: BaseNum + Float> {
    /// Get the current position of the boid
    fn position(&self) -> Vector3<U>;

    /// Get the current velocity of the boid
    fn velocity(&self) -> Vector3<U>;

    /// Get the current acceleration of the boid
    fn acceleration(&self) -> Vector3<U>;

    /// Calculate the separation force for this boid
    fn separate(&self, flock: &Flock<T, U>) -> Vector3<U>;

    /// Calculate the alignment force for this boid
    fn align(&self, flock: &Flock<T, U>) -> Vector3<U>;

    /// Calculate the cohesion force for this boid
    fn cohesion(&self, flock: &Flock<T, U>) -> Vector3<U>;

    /// Set the weights for the boid
    fn set_weights(&mut self, weights: BoidWeights<U>);

    /// Get the weights for the boid
    fn get_weights<'a>(&'a self) -> &'a BoidWeights<U>;

    /// Add another force to the boid and calculate all other internal vectors
    fn with_force(&self, force: Vector3<U>) -> T;

    /// Update the boid based on its flock
    fn update(&self, flock: &Flock<T, U>) -> T;
}
