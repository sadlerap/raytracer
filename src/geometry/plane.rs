use super::*;
use crate::Ray;
use nalgebra::*;

pub struct Plane {
    pub(crate) vertex: Point3<f32>,
    pub(crate) normal: Vector3<f32>,
    pub(crate) material: Material,
}

impl Plane {
    pub fn new(vertex: Point3<f32>, normal: Vector3<f32>, material: Material) -> Plane {
        let normal = normal.normalize();
        Plane {
            vertex,
            normal,
            material,
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let normal = &self.normal;
        let dot = normal.dot(&ray.direction);

        // normal vector of plane and the ray's direction are pointing in the same direction, so
        // they'll never hit
        if dot > 1e-6 {
            return None;
        }

        let v = self.vertex - ray.source;
        let distance = v.dot(normal) / dot;

        if distance > 0.0 {
            let p = ray.source + distance * ray.direction;
            let n = self.normal;
            Some(Intersection::new(distance, p, n))
        } else {
            None
        }
    }
}

impl From<Plane> for Geometry {
    fn from(plane: Plane) -> Self {
        Geometry::Plane(plane)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plane_miss() {
        let p = Plane::new(
            Point3::new(0.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, 0.0),
            Diffuse::new(Color::default(), 1.0).into(),
        );
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 1.0),
        };

        assert!(p.intersect(&ray).is_none());
    }

    #[test]
    fn test_plane_away() {
        let p = Plane::new(
            Point3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Diffuse::new(Color::default(), 1.0).into(),
        );
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(-1.0, 0.0, 0.0),
        };

        assert!(p.intersect(&ray).is_none());
    }

    #[test]
    fn test_plane_hit() {
        let p = Plane::new(
            Point3::new(1.0, 0.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Diffuse::new(Color::default(), 1.0).into(),
        );
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(1.0, 0.0, 0.0),
        };

        assert_eq!(p.intersect(&ray).unwrap().dist, 1.0)
    }

    #[test]
    fn test_plane_origin_behind_ray_source() {
        let p = Plane::new(
            Point3::new(0.0, -1.0, -10.0),
            Vector3::new(0.0, 1.0, 0.0),
            Diffuse::new(Color::default(), 1.0).into(),
        );
        let ray = Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, -1.0, 10.0).normalize(),
        };
        let result = p.intersect(&ray);

        assert!(result.is_some());
    }
}
