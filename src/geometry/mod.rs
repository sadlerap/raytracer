/// All various kinds of scene geometry: Spheres, Planes, etc.
pub mod sphere;

use super::*;
pub use sphere::*;

pub trait Intersectable {
    /// Determines whether the ray will intersect the given object
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}
