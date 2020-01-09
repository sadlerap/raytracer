use nalgebra::*;
use crate::prelude::*;

pub struct Light {
    pub(crate) direction: Vector3<f32>,
    pub(crate) color: Color,
    pub(crate) intensity: f32
}

impl Light {
    pub fn new(direction: Vector3<f32>, color: Color, intensity: f32) -> Light {
        Light {
            direction: direction.normalize(),
            color,
            intensity
        }
    }
}
