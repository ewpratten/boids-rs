use cgmath::{num_traits::Float, BaseNum, InnerSpace, MetricSpace, Vector2, Vector3};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};
use std::ops::{AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

use crate::flock::Flock;

/// Limit the magnitude of a vector
fn limit_magnitude_v2<U: BaseNum + Float>(vector: Vector2<U>, max_magnitude: U) -> Vector2<U> {
    let mag_sq = vector.magnitude2();
    if mag_sq > max_magnitude.powi(2) {
        vector.mul(max_magnitude / mag_sq.sqrt())
    } else {
        vector
    }
}
/// Limit the magnitude of a vector
fn limit_magnitude_v3<U: BaseNum + Float>(vector: Vector3<U>, max_magnitude: U) -> Vector3<U> {
    let mag_sq = vector.magnitude2();
    if mag_sq > max_magnitude.powi(2) {
        vector.mul(max_magnitude / mag_sq.sqrt())
    } else {
        vector
    }
}

/// Defines the force weights for a boid
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct BoidWeights<U: BaseNum> {
    pub alignment: U,
    pub cohesion: U,
    pub separation: U,
}

impl<U: BaseNum + Float> Default for BoidWeights<U> {
    fn default() -> Self {
        Self {
            alignment: U::from(1.5).unwrap(),
            cohesion: U::from(1.0).unwrap(),
            separation: U::from(1.0).unwrap(),
        }
    }
}

/// Common code across all boids no matter their dimensions
pub trait Boid<T: Boid<T, U>, U: BaseNum + Float> {
    /// Get the current position of the boid
    fn position<'a>(&'a self) -> &'a Vector2<U>;

    /// Get the current velocity of the boid
    fn velocity<'a>(&'a self) -> &'a Vector2<U>;

    /// Get the current acceleration of the boid
    fn acceleration<'a>(&'a self) -> &'a Vector2<U>;

    /// Calculate the separation force for this boid
    fn separate(&self, flock: &Flock<T, U>) -> Vector2<U>;

    /// Calculate the alignment force for this boid
    fn align(&self, flock: &Flock<T, U>) -> Vector2<U>;

    /// Calculate the cohesion force for this boid
    fn cohesion(&self, flock: &Flock<T, U>) -> Vector2<U>;

    /// Set the weights for the boid
    fn set_weights(&mut self, weights: BoidWeights<U>);

    /// Get the weights for the boid
    fn get_weights<'a>(&'a self) -> &'a BoidWeights<U>;

    /// Add another force to the boid and calculate all other internal vectors
    fn add_force(&mut self, force: Vector2<U>);

    /// Update the boid based on its flock
    fn update(&mut self, flock: &Flock<T, U>) {
        let weights = self.get_weights();
        let separation = self.separate(flock).mul(weights.separation);
        let alignment = self.align(flock).mul(weights.alignment);
        let cohesion = self.cohesion(flock).mul(weights.cohesion);
        let force = separation + alignment + cohesion;
        self.add_force(force);
    }
}

/// A Boid in 2 dimensions.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Boid2D<U: BaseNum + Float> {
    /// Boid position
    pub position: Vector2<U>,
    /// Boid velocity
    pub velocity: Vector2<U>,
    /// Boid acceleration
    pub acceleration: Vector2<U>,
    /// Boid maximum speed
    pub max_speed: U,
    /// Boid maximum force
    pub max_force: U,
    /// Boid maximum turn rate
    pub r: U,
    /// Boid weights
    pub weights: BoidWeights<U>,
}

impl<U: BaseNum + Float> Boid2D<U> {
    /// Create a new Boid2D from a position and angle
    pub fn new_with_angle(position: Vector2<U>, angle: U) -> Self {
        Self {
            position,
            velocity: Vector2::new(angle.cos(), angle.sin()),
            acceleration: Vector2::new(U::zero(), U::zero()),
            r: U::one() + U::one(),
            max_speed: U::one() + U::one(),
            max_force: U::from(0.03).unwrap(),
            weights: BoidWeights::default(),
        }
    }

    /// Create a new Boid2D from a position and random angle
    pub fn new(position: Vector2<U>) -> Self
    where
        Standard: Distribution<U>,
    {
        let angle = rand::thread_rng().gen::<U>() * U::from(std::f64::consts::PI * 2.0).unwrap();
        Self::new_with_angle(position, angle)
    }
}

impl<T: Boid<T, U>, U: BaseNum + Float> Boid<T, U> for Boid2D<U> {
    fn separate(&self, flock: &Flock<T, U>) -> Vector2<U> {
        // Alloc a steering force
        let mut steer = Vector2::new(U::zero(), U::zero());

        // Tracker for number of boids nearby
        let mut count = U::zero();

        // Steer away from nearby boids
        for boid in flock.boids.iter() {
            let distance = self.position.distance(*boid.position());

            // Only operate on nearby boids
            if distance > U::zero() && distance < flock.goal_separation {
                // Calculate vector pointing away from neighbor
                let diff = (self.position - *boid.position()).normalize().div(distance);
                steer.add_assign(diff);
                count += U::one();
            }
        }

        // Average the steering factor
        if count > U::zero() {
            steer.div_assign(count);
        }

        // Implement Reynolds: Limit the steering force to max_force
        if steer.magnitude() > U::zero() {
            steer = limit_magnitude_v2(
                steer.normalize().mul(self.max_speed).sub(self.velocity),
                self.max_force,
            );
        }

        steer
    }

    fn align(&self, flock: &Flock<T, U>) -> Vector2<U> {
        // Alloc an alignment force
        let mut align = Vector2::new(U::zero(), U::zero());

        // Tracker for number of boids nearby
        let mut count = U::zero();

        // Align with nearby boids
        for boid in flock.boids.iter() {
            let distance = self.position.distance(*boid.position());

            // Only operate on nearby boids
            if distance > U::zero() && distance < flock.goal_alignment {
                align.add_assign(*boid.velocity());
                count += U::one();
            }
        }

        // Average the alignment factor
        if count > U::zero() {
            align.div_assign(count);

            // Implement Reynolds: Limit the steering force to max_force
            limit_magnitude_v2(
                align.normalize().mul(self.max_speed).sub(self.velocity),
                self.max_force,
            )
        } else {
            Vector2::new(U::zero(), U::zero())
        }
    }

    fn cohesion(&self, flock: &Flock<T, U>) -> Vector2<U> {
        // Alloc a steering force
        let mut cohesion = Vector2::new(U::zero(), U::zero());

        // Tracker for number of boids nearby
        let mut count = U::zero();

        // Steer towards nearby boids
        for boid in flock.boids.iter() {
            let distance = self.position.distance(*boid.position());

            // Only operate on nearby boids
            if distance > U::zero() && distance < flock.goal_cohesion {
                cohesion.add_assign(*boid.position());
                count += U::one();
            }
        }

        // Average the cohesion factor
        if count > U::zero() {
            cohesion.div_assign(count);
            cohesion = cohesion.sub(self.position);

            // Implement Reynolds: Limit the steering force to max_force
            limit_magnitude_v2(
                cohesion.normalize().mul(self.max_speed).sub(self.velocity),
                self.max_force,
            )
        } else {
            Vector2::new(U::zero(), U::zero())
        }
    }

    fn set_weights(&mut self, weights: BoidWeights<U>) {
        self.weights = weights;
    }

    fn get_weights<'a>(&'a self) -> &'a BoidWeights<U> {
        &self.weights
    }

    fn add_force(&mut self, force: Vector2<U>) {
        // Apply acceleration to velocity
        self.velocity.add_assign(force);

        // Limit the speed
        self.velocity = limit_magnitude_v2(self.velocity, self.max_speed);

        // Apply velocity to position
        self.position.add_assign(self.velocity);

        // Reset acceleration
        self.acceleration.mul_assign(U::zero());
    }

    fn position<'a>(&'a self) -> &'a Vector2<U> {
        &self.position
    }

    fn velocity<'a>(&'a self) -> &'a Vector2<U> {
        &self.velocity
    }

    fn acceleration<'a>(&'a self) -> &'a Vector2<U> {
        &self.acceleration
    }
}
