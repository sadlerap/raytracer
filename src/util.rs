use nalgebra::*;
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

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        )
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

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Color::new(self.red / other, self.green / other, self.blue / other)
    }
}

/// A ray used in tracing the scene.
pub struct Ray {
    pub(crate) source: Point3<f32>,
    pub(crate) direction: Vector3<f32>,
}

/// Defines a scene
pub struct Scene {
    height: u32,
    width: u32,
    samples: u32,
    fov: f32,
    geometry: Vec<Geometry>,
}

impl Scene {
    pub fn new(width: u32, height: u32, fov: f32, samples: u32) -> Scene {
        Scene {
            height,
            width,
            samples,
            fov,
            geometry: Vec::new(),
        }
    }

    /// Adds a sphere to the scene.
    pub fn add_geometry(&mut self, object: Geometry) {
        self.geometry.push(object);
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

    /// Trace a ray
    fn trace(&self, x: u32, y: u32) -> Color {
        (0..self.samples)
            .map(|_| self.create_camera_ray(x, y))
            .map(|ray| {
                self.geometry
                    .iter()
                    .filter_map(|geom| geom.intersect(&ray).map(|d| (geom, d)))
                    .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                    .map_or_else(|| Color::new(0.0, 0.0, 0.0), |(geom, _)| geom.color())
            })
            .fold(Color::default(), |acc, color| acc + color)
            / self.samples as f32
    }

    pub fn render<T>(&self, writer: &mut T) -> io::Result<()>
    where
        T: Write,
    {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        let colors: Vec<_> = (0..self.width)
            .cartesian_product(0..self.height)
            .par_bridge() // iterate in parallel
            .map(|(x, y)| (x, y, self.trace(x, y)))
            .map(|(x, y, color)| (x, y, color.to_rgb()))
            .collect();

        let sorted = colors.into_iter().sorted_by(|(x1, y1, _), (x2, y2, _)| {
            use std::cmp::Ordering;
            let ycmp = y1.cmp(y2);
            if ycmp != Ordering::Equal {
                ycmp
            } else {
                x1.cmp(x2)
            }
        });

        for (_, _, (red, green, blue)) in sorted {
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
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 1.0), 1.0, Color::new(1.0, 1.0, 1.0));
        let mut scene = Scene::new(4, 3, 90.0, 100);
        scene.add_geometry(sphere.into());
        let result = scene.trace(3, 2);
        assert_eq!(result, color);
    }
}
