use crate::prelude::*;
use nalgebra::*;

pub enum Light {
    Global(GlobalLight),
    Spherical(SphericalLight),
}

impl Colorable for Light {
    fn color(&self, scene: &Scene, i: &Intersection, depth: u32) -> Color {
        match &self {
            Light::Global(gl) => gl.color(scene, i, depth),
            Light::Spherical(sl) => sl.color(scene, i, depth),
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
}

impl Colorable for GlobalLight {
    fn color(&self, scene: &Scene, i: &Intersection, depth: u32) -> Color {
        // basic lambertian lighting
        let light_direction = -self.direction;
        let shadow_ray = Ray::new(i.point, light_direction);
        let visible = scene.trace(&shadow_ray, depth + 1).is_none();
        let intensity = if visible { self.intensity } else { 0.0 };
        let power = i.surface_normal().dot(&light_direction).max(0.0) * intensity;
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
}

impl Colorable for SphericalLight {
    fn color(&self, scene: &Scene, i: &Intersection, depth: u32) -> Color {
        let light_direction = self.pos - i.point;
        let norm = light_direction.norm();
        let shadow_ray = Ray::new(i.point, light_direction);
        let shadow_intersection = scene.trace(&shadow_ray, depth + 1);
        let visible = shadow_intersection.is_none() || shadow_intersection.unwrap().dist > norm;

        let intensity = if visible {
            self.intensity / (4.0 * std::f32::consts::PI * norm)
        } else {
            0.0
        };
        let power = i.surface_normal().dot(&light_direction).max(0.0) * intensity;
        self.color * power
    }
}

impl Into<Light> for SphericalLight {
    fn into(self) -> Light {
        Light::Spherical(self)
    }
}
