extern crate itertools;
extern crate nalgebra;
extern crate raytracer;

use nalgebra::*;
use raytracer::prelude::*;

use std::env;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut scene = Scene::new(1280, 720, 90.0, 1000, Color::default());

    let depth = 7.0;
    let zdepth = 5.0;

    scene.add_light(
        SphericalLight::new(
            Point3::new(0.0, depth - 0.5, 1.5 + zdepth),
            Color::new(1.0, 1.0, 1.0).from_gamma(),
            200.0,
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(4.0, -depth + 1.0, 5.0 + zdepth),
            1.0,
            Diffuse::new(Color::new(0.5, 0.0, 0.2).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(1.0, -depth + 0.8, 4.0 + zdepth),
            0.8,
            Diffuse::new(Color::new(1.0, 0.0, 1.0).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(-3.0, -depth + 2.5, 6.0 + zdepth),
            2.5,
            Diffuse::new(Color::new(0.0, 1.0, 0.0).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Plane::new(
            Point3::new(0.0, -depth, 10.0 + zdepth),
            Vector3::new(0.0, 1.0, 0.0),
            Diffuse::new(Color::new(0.4, 0.1, 0.3).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Sphere::new(
            Point3::new(1.0, -depth + 1.5, 6.0 + zdepth),
            1.5,
            Reflective::new(Color::new(0.1, 0.0, 0.1).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Plane::new(
            Point3::new(0.0, 0.0, 15.0 + zdepth),
            Vector3::new(0.0, 0.0, -1.0),
            Diffuse::new(Color::new(1.0, 1.0, 1.0).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Plane::new(
            Point3::new(-10.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Diffuse::new(Color::new(1.0, 0.0, 0.0).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Plane::new(
            Point3::new(10.0, 0.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Diffuse::new(Color::new(0.0, 0.0, 1.0).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    scene.add_geometry(
        Plane::new(
            Point3::new(0.0, depth, 10.0 + zdepth),
            Vector3::new(0.0, -1.0, 0.0),
            Diffuse::new(Color::new(1.0, 1.0, 1.0).from_gamma(), 0.18).into(),
        )
        .into(),
    );

    let mut file = env::args()
        .nth(1)
        .map_or_else(|| File::create("./output.ppd"), File::create)
        .map(io::BufWriter::new)?;

    scene.render(&mut file)
}
