use crate::prelude::*;
use nalgebra::*;

pub enum Light {
    Global(GlobalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn light(&self, scene: &Scene, i: &Intersection) -> Color {
        match &self {
            Light::Global(gl) => gl.light(scene, i),
            Light::Spherical(sl) => sl.light(scene, i),
        }
    }
}

#[derive(Debug)]
pub struct GlobalLight {
    pub(crate) direction: Vector3<f32>,
    pub(crate) color: Color,
    pub(crate) intensity: f32,
}

impl GlobalLight {
    pub fn new(direction: Vector3<f32>, color: Color, intensity: f32) -> GlobalLight {
        GlobalLight {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }

    pub fn light(&self, scene: &Scene, i: &Intersection) -> Color {
        // basic lambertian lighting
        let light_direction = -self.direction;
        let shadow_ray = Ray::new(i.point, light_direction);
        let visible = !scene
            .geometry
            .iter()
            .map(|g| g.intersect(&shadow_ray))
            .any(|r| r.is_some());
        let intensity = if visible { self.intensity } else { 0.0 };
        let power = i.normal.dot(&light_direction).max(0.0) * intensity;
        self.color * power
    }
}

impl Into<Light> for GlobalLight {
    fn into(self) -> Light {
        Light::Global(self)
    }
}

pub struct SphericalLight {
    pos: Point3<f32>,
    color: Color,
    intensity: f32,
}

impl SphericalLight {
    pub fn new(pos: Point3<f32>, color: Color, intensity: f32) -> SphericalLight {
        SphericalLight {
            pos,
            color,
            intensity,
        }
    }

    pub fn light(&self, scene: &Scene, i: &Intersection) -> Color {
        let light_direction = self.pos - i.point;
        let norm = light_direction.norm();
        let shadow_ray = Ray::new(i.point, light_direction);
        let shadow_intersection = scene
            .geometry
            .iter()
            .filter_map(|g| g.intersect(&shadow_ray))
            .min_by(|i1, i2| (&i1.dist).partial_cmp(&i2.dist).unwrap());
        let visible = shadow_intersection.is_none() || shadow_intersection.unwrap().dist > norm;

        let intensity = if visible {
            self.intensity / (4.0 * std::f32::consts::PI * norm)
        } else {
            0.0
        };
        let power = i.normal.dot(&light_direction).max(0.0) * intensity;
        self.color * power
    }
}

impl Into<Light> for SphericalLight {
    fn into(self) -> Light {
        Light::Spherical(self)
    }
}
