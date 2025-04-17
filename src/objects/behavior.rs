//! Traits for Raytracer Objects

use super::circle::ObjectCircle;
use super::emitters::{Emitter, EmitterCollimated};

/// Enum for all Raytracer Objects
pub enum RaytracerObjects {
    ObjectCircle(ObjectCircle),
    Emitter(Emitter),
    EmitterCollimated(EmitterCollimated),
}

/// Drawable trait for Objects
pub trait Drawable {
    fn draw_object(&self);
}

/// Movable trait for Objects
pub trait Movable {
    fn move_object(&mut self, pos_x: f32, pos_y: f32);
}