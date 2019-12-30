extern crate itertools;
extern crate nalgebra;

use nalgebra::*;
use raytracer::prelude::*;

use std::env;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut scene = Scene::new(1920, 1080, 90.0, 100);

    scene.add_geometry(
        Sphere::new(
            Point3::new(0.0, 0.0, 5.0),
            1.0,
            Diffuse::new(Color::new(0.5, 0.0, 0.2), 0.8).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(1.0, 1.0, 4.0),
            0.8,
            Diffuse::new(Color::new(1.0, 0.0, 1.0), 0.6).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(2.0, 1.0, 9.0),
            2.5,
            Diffuse::new(Color::new(0.0, 1.0, 0.0), 0.9).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Plane::new(
            Point3::new(0.0, -5.0, 10.0),
            Vector3::new(0.0, 1.0, 0.0),
            Diffuse::new(Color::new(0.2, 0.2, 1.0), 0.5).into(),
        )
        .into(),
    );

    let mut file = env::args()
        .nth(1)
        .map_or_else(|| File::create("./output.ppd"), File::create)
        .map(io::BufWriter::new)?;

    scene.render(&mut file)?;
    Ok(())
}
