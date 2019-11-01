use image::{DynamicImage, GenericImage, Pixel};
use itertools::Itertools;
use nalgebra::*;

use geometry::*;

pub mod geometry;

/// Stores a color using `f64`s
#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    /// Constructs a new color
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        )
    }
}

pub struct Ray {
    source: Point3<f64>,
    direction: Vector3<f64>,
}

pub struct Scene {
    height: u32,
    width: u32,
    fov: f64,
    geometry: Vec<Sphere>,
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f64) -> Scene {
        Scene {
            height,
            width,
            fov,
            geometry: Vec::new(),
        }
    }

    /// Adds a sphere to the scene.
    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.geometry.push(sphere);
    }

    /// Creates a prime ray for the pixel at the coordinate (x, y) in image space.  This uses the
    /// convention that (0, 0) in image space corresponds to the upper left corner
    fn create_camera_ray(&self, x: u32, y: u32) -> Ray {
        // for now, assume that images are wider than they are tall
        assert!(self.height < self.width);
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f64) / (self.height as f64);

        let camera_x =
            (((x as f64 + 0.5) / self.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let camera_y = 1.0 - ((y as f64 + 0.5) / self.height as f64) * 2.0;
        Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(camera_x, camera_y, 1.0).normalize(),
        }
    }

    fn trace(&self, x: u32, y: u32) -> Color {
        let ray = self.create_camera_ray(x, y);
        self.geometry
            .iter()
            .filter_map(|sphere| sphere.intersect(&ray).map(|d| (sphere, d)))
            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
            .map_or_else(
                || Color::new(0.0, 0.0, 0.0),
                |(sphere, _)| sphere.color.clone(),
            )
    }

    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| (x, y, self.trace(x, y).to_rgb()))
            .for_each(|(x, y, (red, green, blue))| {
                image.put_pixel(x, y, Pixel::from_channels(red, green, blue, 255))
            });
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let color = Color::new(1.0, 1.0, 1.0);
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 1.0), 1.0, Color::new(1.0, 1.0, 1.0));
        let mut scene = Scene::new(4, 3, 90.0);
        scene.add_sphere(sphere);
        let result = scene.trace(3, 2);
        assert_eq!(result, color);
    }
}
