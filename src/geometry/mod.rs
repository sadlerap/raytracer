pub mod plane;
/// All various kinds of scene geometry: Spheres, Planes, etc.
pub mod sphere;

use crate::prelude::*;
use nalgebra::*;
pub use plane::*;
pub use sphere::*;

pub trait Intersectable {
    /// Determines whether the ray will intersect the given object
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn surface_normal(&self, hit_point: &Point3<f32>) -> Vector3<f32>;
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub(crate) dist: f32,
    pub(crate) point: Point3<f32>,
    pub(crate) elem: &'a Geometry,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(
        dist: f32,
        ray: &Ray,
        elem: &'a Geometry,
    ) -> Intersection<'b>
        where 'a: 'b
    {
        Intersection {
            dist,
            point: ray.source + dist * ray.direction,
            elem,
        }
    }

    pub fn surface_normal(&self) -> Vector3<f32> {
        self.elem.surface_normal(&self.point)
    }
}

#[derive(Debug)]
pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
}

impl Colorable for Geometry {
    fn color(&self, scene: &Scene, i: Intersection, tracing_depth: u32) -> Color {
        match self {
            Geometry::Sphere(s) => s.color(scene, i, tracing_depth),
            Geometry::Plane(p) => p.color(scene, i, tracing_depth),
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

    fn surface_normal(&self, hit_point: &Point3<f32>) -> Vector3<f32> {
        match self {
            Geometry::Sphere(s) => s.surface_normal(hit_point),
            Geometry::Plane(p) => p.surface_normal(hit_point),
        }
    }
}
