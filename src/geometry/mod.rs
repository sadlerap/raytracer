pub mod plane;
/// All various kinds of scene geometry: Spheres, Planes, etc.
pub mod sphere;

use super::*;
pub use plane::*;
pub use sphere::*;

pub trait Intersectable {
    /// Determines whether the ray will intersect the given object
    fn intersect(&self, ray: &Ray) -> Option<f32>;
}
