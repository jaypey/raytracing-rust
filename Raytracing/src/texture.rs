use nalgebra::Vector3;

use nalgebra;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32>;
}

pub struct SolidColor {
    Color: Vector3<f32>,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        SolidColor {
            Color: Vector3::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        self.Color
    }
}

pub struct CheckerTexture<C1: Texture, C2: Texture> {
    pair: C1,
    impair: C2,
}

impl<C1: Texture, C2: Texture> CheckerTexture<C1, C2> {
    pub fn new(pair: C1, impair: C2) -> Self {
        CheckerTexture {
            pair: pair,
            impair: impair,
        }
    }
}

impl<C1: Texture, C2: Texture> Texture for CheckerTexture<C1, C2> {
    fn value(&self, u: f32, v: f32, p: &Vector3<f32>) -> Vector3<f32> {
        let sines = f32::sin(10.0 * p.x) * f32::sin(10.0 * p.y) * f32::sin(10.0 * p.z);
        if sines < 0.0 {
            self.pair.value(u, v, p)
        } else {
            self.impair.value(u, v, p)
        }
    }
}
