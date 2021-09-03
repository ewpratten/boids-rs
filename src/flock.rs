use std::marker::PhantomData;

use cgmath::{
    num_traits::{Float, Num},
    BaseNum,
};

use crate::boid::Boid;

pub struct Flock<T: Boid<T, U>, U: BaseNum + Float> {
    pub boids: Vec<T>,
    pub goal_separation: U,
    pub goal_alignment: U,
    pub goal_cohesion: U,
    _phantom: PhantomData<U>,
}

impl<T: Boid<T, U>, U: BaseNum + Float> Flock<T, U> {
    // pub fn new(count: usize) -> Self {

    // }
}
