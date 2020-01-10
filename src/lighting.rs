use crate::prelude::*;
use nalgebra::*;

#[derive(Debug)]
pub struct GlobalLight {
    pub(crate) direction: Vector3<f32>,
    pub(crate) color: Color,
    pub(crate) intensity: f32,
}

impl GlobalLight {
    pub fn new(direction: Vector3<f32>, color: Color, intensity: f32) -> GlobalLight {
        GlobalLight {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }
}
