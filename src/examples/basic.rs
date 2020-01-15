extern crate itertools;
extern crate nalgebra;
extern crate raytracer;

use nalgebra::*;
use raytracer::prelude::*;

use std::env;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut scene = Scene::new(1280, 720, 90.0, 500, Color::default());

    let depth = -1.5;

    scene.add_light(
        SphericalLight::new(
            Point3::new(1.0, 1.0, 1.0),
            Color::new(1.0, 1.0, 1.0).from_gamma(),
            100.0,
        )
        .into(),
    );

    scene.add_light(
        GlobalLight::new(
            Vector3::new(1.0, -1.0, 1.0),
            Color::new(0.9, 1.0, 0.5).from_gamma(),
            15.0,
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(4.0, depth + 1.0, 5.0),
            1.0,
            Diffuse::new(Color::new(0.5, 0.0, 0.2).from_gamma(), 0.4).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(1.0, depth + 0.8, 4.0),
            0.8,
            Diffuse::new(Color::new(1.0, 0.0, 1.0).from_gamma(), 0.3).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(-3.0, depth + 2.5, 6.0),
            2.5,
            Diffuse::new(Color::new(0.0, 1.0, 0.0).from_gamma(), 0.3).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Plane::new(
            Point3::new(0.0, depth, 10.0),
            Vector3::new(0.0, 1.0, 0.0),
            Diffuse::new(Color::new(0.4, 0.1, 0.3).from_gamma(), 0.4).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(1.0, depth + 1.5, 6.0),
            1.5,
            Reflective::new(Color::new(0.1, 0.0, 0.1).from_gamma(), 0.4).into(),
        )
        .into(),
    );

    let mut file = env::args()
        .nth(1)
        .map_or_else(|| File::create("./output.ppd"), File::create)
        .map(io::BufWriter::new)?;

    scene.render(&mut file)
}
