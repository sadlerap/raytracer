use crate::prelude::*;
use nalgebra::*;
use rand::{distributions::*, prelude::*};

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
        let normal = i.surface_normal();
        let sphere_center = i.point + normal;

        let point = {
            let dist = Uniform::new_inclusive(-1.0, 1.0);
            let mut rng = thread_rng();
            let mut point = Vector3::new(
                dist.sample(&mut rng),
                dist.sample(&mut rng),
                dist.sample(&mut rng),
            );
            while point.magnitude_squared() >= 1.0 {
                point = Vector3::new(
                    dist.sample(&mut rng),
                    dist.sample(&mut rng),
                    dist.sample(&mut rng),
                );
            }
            sphere_center + point
        };

        let direction = point - i.point;
        let secondary_ray = Ray::new(
            // step away from the surface to prevent "pox" from showing up
            i.point + 1e-6 * direction,
            direction,
        );
        let traced_color = scene
            .trace(&secondary_ray, depth + 1)
            .map_or_else(|| scene.background, |i| i.elem.color(scene, i, depth + 1));
        let surface_color = self.color.lerp(traced_color, self.albedo);

        let reflected = self.albedo / std::f32::consts::PI;

        scene
            .lights
            .iter()
            .map(|l| l.light(scene, &i))
            .map(|c| surface_color * c * reflected)
            .fold(Color::default(), |acc, item| acc + item)
            .clamp()
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
