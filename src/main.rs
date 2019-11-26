extern crate image;
extern crate itertools;
extern crate nalgebra;

use nalgebra::*;
use raytracer::geometry::*;
use raytracer::*;

use std::env;

fn main() {
    let mut scene = Scene::new(1920, 1080, 90.0);

    scene.add_geometry(
        Sphere::new(Point3::new(0.0, 0.0, 5.0), 1.0, Color::new(0.5, 0.0, 0.2)).into(),
    );
    scene.add_geometry(
        Sphere::new(Point3::new(1.0, 1.0, 4.0), 0.8, Color::new(1.0, 0.0, 1.0)).into(),
    );
    scene.add_geometry(
        Sphere::new(Point3::new(2.0, 1.0, 9.0), 2.5, Color::new(0.0, 1.0, 0.0)).into(),
    );
    scene.add_geometry(
        Plane::new(
            Point3::new(0.0, -5.0, 10.0),
            Vector3::new(0.0, 1.0, 0.0),
            Color::new(0.2, 0.2, 1.0),
        )
        .into(),
    );

    let image = scene.render();

    let file = env::args()
        .nth(1)
        .unwrap_or_else(|| "./output.png".to_string());
    image.save(file).expect("Unable to save file");
}
