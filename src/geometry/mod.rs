pub mod plane;
/// All various kinds of scene geometry: Spheres, Planes, etc.
pub mod sphere;

use crate::prelude::*;
use nalgebra::*;
pub use plane::*;
pub use sphere::*;

pub trait Intersectable {
    /// Determines whether the ray will intersect the given object
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Debug)]
pub struct Intersection {
    pub(crate) dist: f32,
    pub(crate) point: Point3<f32>,
    pub(crate) normal: Vector3<f32>,
}

impl Intersection {
    fn new(dist: f32, point: Point3<f32>, normal: Vector3<f32>) -> Intersection {
        Intersection {
            dist: dist,
            point: point,
            normal: normal.normalize(),
        }
    }
}

pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
}

impl Colorable for Geometry {
    fn color(&self, scene: &Scene, i: Intersection, tracing_depth: u32) -> Color {
        match self {
            Geometry::Sphere(s) => s.material.color(scene, i, tracing_depth),
            Geometry::Plane(p) => p.material.color(scene, i, tracing_depth),
        }
    }
}

impl Intersectable for Geometry {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Geometry::Sphere(sphere) => sphere.intersect(ray),
            Geometry::Plane(plane) => plane.intersect(ray),
        }
    }
}
