/// All various kinds of scene geometry: Spheres, Planes, etc.
use nalgebra::Point3;

use super::*;

pub trait Intersectable {
    /// Determines whether the ray will intersect the given object
    fn intersect(&self, ray: &Ray) -> bool;
}

trait Geometry {
    fn position(&self) -> Point3<f64>;
}

/// A sphere.
pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, color: Color) -> Sphere {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

impl Intersectable for Sphere {
    /// Determines whether the ray will intersect based on geometry
    fn intersect(&self, ray: &Ray) -> bool {
        // length of leg a of the triangle
        let direct_distance = &self.center - &ray.source;
        // length of the hypotenuse
        let adjacent_leg = direct_distance.dot(&ray.direction);
        // length of the remaining side (squared)
        let d = direct_distance.dot(&direct_distance) - adjacent_leg.powi(2);

        d < self.radius.powi(2)
    }
}
