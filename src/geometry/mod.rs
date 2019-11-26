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

pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
}

impl Geometry {
    pub fn color(&self) -> Color {
        match self {
            Geometry::Sphere(s) => s.color,
            Geometry::Plane(p) => p.color,
        }
    }
}

impl Intersectable for Geometry {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        match self {
            Geometry::Sphere(sphere) => sphere.intersect(ray),
            Geometry::Plane(plane) => plane.intersect(ray),
        }
    }
}
