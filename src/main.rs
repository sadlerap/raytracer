extern crate image;
extern crate itertools;
extern crate nalgebra;

use nalgebra::Point3;
use raytracer::geometry::*;
use raytracer::*;

use std::env;

fn main() {
    let mut scene = Scene::new(1920, 1080, 90.0);
    scene.add_sphere(Sphere::new(
        Point3::new(0.0, 0.0, 5.0),
        1.0,
        Color::new(0.5, 0.5, 0.0),
    ));
    let image = scene.render();

    let file = env::args()
        .nth(1)
        .unwrap_or_else(|| "./output.png".to_string());
    image.save(file).expect("Unable to save file");
}
