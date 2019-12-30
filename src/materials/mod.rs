use crate::prelude::*;
use nalgebra::*;
use rand::*;
use rayon::prelude::*;

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

        let gen_ray = || {
            let mut point = (
                random::<f32>().mul_add(2.0, -1.0),
                random::<f32>().mul_add(2.0, -1.0),
                random::<f32>().mul_add(2.0, -1.0),
            );
            while (point.0 * point.0 + point.1 * point.1 + point.2 * point.2) >= 1.0 {
                point = (
                    random::<f32>().mul_add(2.0, -1.0),
                    random::<f32>().mul_add(2.0, -1.0),
                    random::<f32>().mul_add(2.0, -1.0),
                );
            }
            let point = sphere_center + Vector3::new(point.0, point.1, point.2);

            Ray {
                source: i.point,
                direction: (point - i.point).normalize(),
            }
        };

        let traced_color = (0..scene.samples)
            .into_par_iter()
            .map(|_| gen_ray())
            .map(|ray| scene.trace(&ray, depth + 1))
            .reduce(|| Color::default(), |acc, color| acc + color)
            / scene.samples as f32;

        self.color.lerp(traced_color, self.albedo)
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
