use super::*;
use crate::Ray;
use nalgebra::*;

pub struct Plane {
    vertex: Point3<f32>,
    normal: Vector3<f32>,
}

impl Plane {
    pub fn new(vertex: Point3<f32>, normal: Vector3<f32>) -> Plane {
        let normal = normal.normalize();
        Plane { vertex, normal }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let normal = &self.normal;
        let dot = normal.dot(&ray.direction);
        if dot < 1e-6 {
            return None;
        }

        let v = &self.vertex - &ray.source;
        let distance = v.dot(normal) / dot;

        if distance > 0.0 {
            Some(distance)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plane_miss() {
        let p = Plane::new(Point3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };

        assert_eq!(p.intersect(&ray), None);
    }

    #[test]
    fn test_plane_away() {
        let p = Plane::new(Point3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-1.0, 0.0, 0.0),
        };

        assert_eq!(p.intersect(&ray), None);
    }

    #[test]
    fn test_plane_hit() {
        let p = Plane::new(Point3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
        };

        assert_eq!(p.intersect(&ray), Some(1.0))
    }
}
