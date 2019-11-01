use nalgebra::{Point3, Vector3};

use crate::{geometry::Intersectable, Color, Ray};

/// A sphere.
#[derive(Debug)]
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
    /// Determines whether the ray will intersect the sphere. See
    /// [here](https://bheisler.github.io/post/writing-raytracer-in-rust-part-2/)
    /// for more information on how this works.
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // length of leg a of the triangle
        let direct_distance: Vector3<_> = self.center - ray.source;
        // length of the hypotenuse
        let adjacent_leg = direct_distance.dot(&ray.direction);
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
            Some(t0.min(t1))
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use super::*;

    #[test]
    fn test_intersect() {
        let sphere = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, Color::new(1.0, 1.0, 1.0));
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(1.0, 1.0, 1.0).normalize(),
        };
        let result = sphere.intersect(&ray);
        assert!(result.unwrap() - (3.0 as f64).sqrt() - 1.0 <= 1e-6);
    }

    #[test]
    fn test_near_miss() {
        let sphere = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0, Color::new(1.0, 1.0, 1.0));
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };
        assert!(sphere.intersect(&ray).is_none());
    }
}