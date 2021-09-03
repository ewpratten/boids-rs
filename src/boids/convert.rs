use cgmath::{num_traits::Float, Vector2, Vector3};

pub trait LossyConvert<T> {
    fn lossy_convert(&self) -> T;
}

impl<U: Clone> LossyConvert<Vector2<U>> for Vector3<U> {
    fn lossy_convert(&self) -> Vector2<U> {
        Vector2::new(self.x.clone(), self.y.clone())
    }
}

impl<U: Float> LossyConvert<Vector3<U>> for Vector2<U> {
    fn lossy_convert(&self) -> Vector3<U> {
        Vector3::new(self.x, self.y, U::zero())
    }
}
