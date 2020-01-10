use crate::prelude::*;
use nalgebra::*;
use rand::*;

/// Colors a scene
pub trait Colorable {
    fn color(&self, scene: &Scene, intersection: Intersection, depth: u32) -> Color;
}

/// A diffuse material.  Absorbs light in each ray bounce based on the albedo.
#[derive(Debug)]
pub struct Diffuse {
    color: Color,
    albedo: f32,
}

impl Diffuse {
    pub fn new(color: Color, albedo: f32) -> Diffuse {
        Diffuse { color, albedo }
    }
}

impl Colorable for Diffuse {
    fn color(&self, scene: &Scene, i: Intersection, depth: u32) -> Color {
        let sphere_center = i.point + i.normal;

        let point = {
            let mut point = Vector3::new(
                random::<f32>().mul_add(2.0, -1.0),
                random::<f32>().mul_add(2.0, -1.0),
                random::<f32>().mul_add(2.0, -1.0),
            );
            while point.magnitude_squared() >= 1.0 {
                point = Vector3::new(
                    random::<f32>().mul_add(2.0, -1.0),
                    random::<f32>().mul_add(2.0, -1.0),
                    random::<f32>().mul_add(2.0, -1.0),
                );
            }
            sphere_center + point
        };

        let direction = (point - i.point).normalize();
        let secondary_ray = Ray {
            // step away from the surface to prevent "pox" from showing up
            source: i.point + 1e-6 * direction,
            direction,
        };
        let traced_color = scene.trace(&secondary_ray, depth + 1);
        let surface_color = self.color.lerp(traced_color, self.albedo);
        let mut color = Color::default();

        let reflected = self.albedo / std::f32::consts::PI;
        for l in &scene.lights {
            color += surface_color * l.light(scene, &i) * reflected;
        }
        color.clamp()
    }
}

impl Into<Material> for Diffuse {
    fn into(self) -> Material {
        Material::Diffuse(self)
    }
}

/// A generic material.  Allows objects to have any kind of material supported.
#[derive(Debug)]
pub enum Material {
    Diffuse(Diffuse),
}

impl Colorable for Material {
    fn color(&self, scene: &Scene, i: Intersection, depth: u32) -> Color {
        match self {
            Material::Diffuse(x) => x.color(scene, i, depth),
        }
    }
}
