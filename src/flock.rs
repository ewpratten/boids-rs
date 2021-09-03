use cgmath::{num_traits::Float, BaseNum};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::marker::PhantomData;

use crate::boids::Boid;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Flock<T: Boid<T, U>, U: BaseNum + Float> {
    pub boids: Vec<T>,
    pub goal_separation: U,
    pub goal_alignment: U,
    pub goal_cohesion: U,
    _phantom: PhantomData<U>,
}

impl<T: Boid<T, U> + Clone, U: BaseNum + Float> Flock<T, U>
where
    [T]: Sized,
    T: std::marker::Send,
    T: std::marker::Sync,
    U: std::marker::Sync,
{
    /// Update all boids in the flock
    pub fn update(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "rayon")] {
                self.boids = self.boids
                    .par_iter()
                    .map(|boid| boid.update(&self.clone()))
                    .collect();
            } else {
                self.boids = self.boids
                    .iter()
                    .map(|boid| boid.update(&self.clone()))
                    .collect();
            }
        }
    }
}

impl<T: Boid<T, U>, U: BaseNum + Float> Default for Flock<T, U> {
    fn default() -> Self {
        Self {
            boids: Default::default(),
            goal_separation: U::from(1.5).unwrap(),
            goal_alignment: U::one(),
            goal_cohesion: U::one(),
            _phantom: Default::default(),
        }
    }
}
