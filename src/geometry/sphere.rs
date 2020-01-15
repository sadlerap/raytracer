use nalgebra::*;

use super::*;
use crate::prelude::*;

/// A sphere.
#[derive(Debug)]
pub struct Sphere {
    pub(crate) center: Point3<f32>,
    pub(crate) radius: f32,
    pub(crate) material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Colorable for Sphere {
    fn color(&self, scene: &Scene, i: &Intersection, tracing_depth: u32) -> Color {
        self.material.color(scene, i, tracing_depth)
    }
}

impl Intersectable for Sphere {
    /// Determines whether the ray will intersect the sphere. See
    /// [here](https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/)
    /// for more information on how this works.
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        // length of leg a of the triangle
        let direct_distance = self.center - ray.source;
        // length of the hypotenuse
        let adjacent_leg = direct_distance.dot(&ray.direction);
        if adjacent_leg.signum() < 0.0 {
            return None;
        }

        // length of the remaining side (squared)
        let d = direct_distance.dot(&direct_distance) - adjacent_leg.powi(2);
        let radius2 = self.radius.powi(2);

        if d >= radius2 {
            return None;
        }

        let thickness = (radius2 - d).sqrt();
        let t0 = adjacent_leg + thickness;
        let t1 = adjacent_leg - thickness;

        if t0 < 0.0 && t1 < 0.0 {
            None
        } else {
            let d = if t0 < 0.0 {
                t1
            } else if t1 < 0.0 {
                t0
            } else {
                t0.min(t1)
            };
            Some(d)
        }
    }

    fn surface_normal(&self, hit_point: &Point3<f32>) -> Vector3<f32> {
        (hit_point - self.center).normalize()
    }
}

impl From<Sphere> for Geometry {
    fn from(s: Sphere) -> Self {
        Geometry::Sphere(s)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use super::*;

    #[test]
    fn test_intersect() {
        let sphere = Sphere::new(
            Point3::new(1.0, 1.0, 1.0),
            1.0,
            Diffuse::new(Color::new(1.0, 1.0, 1.0), 1.0).into(),
        );
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(1.0, 1.0, 1.0).normalize(),
        };
        let result = sphere.intersect(&ray);
        assert!(result.unwrap() - (3.0 as f32).sqrt() - 1.0 <= 1e-6);
    }

    #[test]
    fn test_near_miss() {
        let sphere = Sphere::new(
            Point3::new(1.0, 1.0, 1.0),
            1.0,
            Diffuse::new(Color::new(1.0, 1.0, 1.0), 1.0).into(),
        );
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        assert!(sphere.intersect(&ray).is_none());
    }
}
