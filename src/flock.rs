use cgmath::{num_traits::Float, BaseNum};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::boids::Boid;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Flock<T: Boid<T, U>, U: BaseNum + Float> {
    pub boids: Vec<T>,
    pub goal_separation: U,
    pub goal_alignment: U,
    pub goal_cohesion: U,
}

impl<T: Boid<T, U> + Clone, U: BaseNum + Float> Flock<T, U>
where
    T: std::marker::Send,
    T: std::marker::Sync,
    U: std::marker::Sync,
{
    /// Update all boids in the flock
    pub fn update(&mut self) {
        #[cfg(feature = "puffin")]
        puffin::profile_function!();

        // Handle weather we are running parallel or single-thread
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
            boids: Vec::new(),
            goal_separation: U::from(25.0).unwrap(),
            goal_alignment: U::from(50.0).unwrap(),
            goal_cohesion: U::from(50.0).unwrap(),
        }
    }
}
