use itertools::Itertools;
use nalgebra::*;
use rand::prelude::*;
use rayon::prelude::*;

use std::io;
use std::io::prelude::*;
use std::ops::*;

use crate::prelude::*;

/// Stores a color using `f32`s
#[derive(Clone, Debug, PartialEq, Default, Copy)]
pub struct Color {
    pub(crate) red: f32,
    pub(crate) green: f32,
    pub(crate) blue: f32,
}
impl Color {
    /// Constructs a new color
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn lerp(&self, other: Color, amount: f32) -> Color {
        *self * amount + other * (1.0 - amount)
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        )
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
        }
    }

    pub const GAMMA: f32 = 2.2;
    pub const INV_GAMMA: f32 = 1.0 / 2.2;

    pub fn from_gamma(&self) -> Color {
        Color {
            red: self.red.powf(Color::GAMMA),
            green: self.green.powf(Color::GAMMA),
            blue: self.blue.powf(Color::GAMMA),
        }
    }

    pub fn to_gamma(&self) -> Color {
        Color {
            red: self.red.powf(Color::INV_GAMMA),
            green: self.green.powf(Color::INV_GAMMA),
            blue: self.blue.powf(Color::INV_GAMMA),
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Self::Output {
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        other * self
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.red *= rhs.red;
        self.green *= rhs.green;
        self.blue *= rhs.blue;
    }
}

impl Div<f32> for Color {
    type Output = Color;
    fn div(self, other: f32) -> Self::Output {
        Color::new(self.red / other, self.green / other, self.blue / other)
    }
}

/// A ray used in tracing the scene.
#[derive(Debug, Clone)]
pub struct Ray {
    pub(crate) source: Point3<f32>,
    pub(crate) direction: Vector3<f32>,
}

impl Ray {
    pub fn new(source: Point3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            source,
            direction: direction.normalize(),
        }
    }

    pub fn reflect(
        normal: Vector3<f32>,
        incident: Vector3<f32>,
        intersection: &Point3<f32>,
        bias: f32,
    ) -> Ray {
        Ray::new(
            intersection + (normal * bias),
            incident - (2.0 * incident.dot(&normal) * normal),
        )
    }
}

/// Defines a scene
pub struct Scene {
    pub(crate) height: u32,
    pub(crate) width: u32,
    pub(crate) samples: u32,
    pub(crate) fov: f32,
    pub(crate) background: Color,
    pub(crate) geometry: Vec<Geometry>,
    pub(crate) tracing_depth: u32,
    pub(crate) lights: Vec<Light>,
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f32, samples: u32, background: Color) -> Scene {
        Scene {
            height,
            width,
            samples,
            fov,
            background,
            geometry: Vec::new(),
            tracing_depth: 3,
            lights: Vec::new(),
        }
    }

    /// Adds an object to the scene.
    pub fn add_geometry<G>(&mut self, object: G)
    where
        G: Into<Geometry>,
    {
        self.geometry.push(object.into());
    }

    pub fn add_light<L>(&mut self, light: L)
    where
        L: Into<Light>,
    {
        self.lights.push(light.into());
    }

    /// Creates a prime ray for the pixel at the coordinate (x, y) in image space.  This uses the
    /// convention that (0, 0) in image space corresponds to the upper left corner
    fn create_camera_ray(&self, x: u32, y: u32) -> Ray {
        // for now, assume that images are wider than they are tall
        assert!(self.height < self.width);
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (self.width as f32) / (self.height as f32);

        let camera_x = (((x as f32 + random::<f32>()) / self.width as f32) * 2.0 - 1.0)
            * aspect_ratio
            * fov_adjustment;
        let camera_y = 1.0 - ((y as f32 + random::<f32>()) / self.height as f32) * 2.0;
        Ray {
            source: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(camera_x, camera_y, 1.0).normalize(),
        }
    }

    pub(crate) fn trace<'a>(&'a self, ray: &'a Ray, depth: u32) -> Option<Intersection<'a>> {
        if depth >= self.tracing_depth {
            None
        } else {
            self.geometry
                .iter()
                .filter_map(|g| g.intersect(&ray).map(|i| Intersection::new(i, &ray, g)))
                .min_by(|i1, i2| (&i1.dist).partial_cmp(&i2.dist).unwrap())
        }
    }

    /// Trace a ray
    fn trace_scene_ray(&self, x: u32, y: u32) -> Color {
        let mut color = Color::default();
        for _ in 0..self.samples {
            let ray = self.create_camera_ray(x, y);
            color += self
                .trace(&ray, 0)
                .map_or_else(|| self.background, |i| i.elem.color(self, &i, 0));
        }
        color / self.samples as f32
    }

    pub fn render<T>(&self, writer: &mut T) -> io::Result<()>
    where
        T: Write,
    {
        let sorted = (0..self.width)
            .cartesian_product(0..self.height)
            .par_bridge() // iterate in parallel
            .map(|(x, y)| (x, y, self.trace_scene_ray(x, y)))
            .map(|(x, y, color)| (x, y, color.to_gamma().to_rgb()))
            .collect::<Vec<_>>()
            .into_iter()
            .sorted_by(|(x1, y1, _), (x2, y2, _)| {
                use std::cmp::Ordering;
                let ycmp = y1.cmp(y2);
                if ycmp == Ordering::Equal {
                    x1.cmp(x2)
                } else {
                    ycmp
                }
            })
            .map(|(_, _, colors)| colors);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for (red, green, blue) in sorted {
            writeln!(writer, "{} {} {}", red, green, blue)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let color = Color::new(1.0, 1.0, 1.0);
        let sphere = Sphere::new(
            Point3::new(0.0, 0.0, 1.0),
            1.0,
            Diffuse::new(Color::new(1.0, 1.0, 1.0), 1.0).into(),
        );
        let mut scene = Scene::new(4, 3, 90.0, 100, Color::default());
        scene.add_geometry(sphere);
        let result = scene.trace_scene_ray(3, 2);
        assert_eq!(result, color);
    }
}
