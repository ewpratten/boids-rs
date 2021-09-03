use std::ops::Mul;

use cgmath::{BaseNum, InnerSpace, Vector2, Vector3, num_traits::Float};

/// Limit the magnitude of a vector
pub fn limit_magnitude_v2<U: BaseNum + Float>(vector: Vector2<U>, max_magnitude: U) -> Vector2<U> {
    let mag_sq = vector.magnitude2();
    if mag_sq > max_magnitude.powi(2) {
        vector.mul(max_magnitude / mag_sq.sqrt())
    } else {
        vector
    }
}

/// Limit the magnitude of a vector
pub fn limit_magnitude_v3<U: BaseNum + Float>(vector: Vector3<U>, max_magnitude: U) -> Vector3<U> {
    let mag_sq = vector.magnitude2();
    if mag_sq > max_magnitude.powi(2) {
        vector.mul(max_magnitude / mag_sq.sqrt())
    } else {
        vector
    }
}
