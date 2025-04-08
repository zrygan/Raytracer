//! Traits for Raytracer Objects
use super::circle::ObjectCircle;

/// Enum for all Raytracer Objects
pub enum RaytracerObjects {
    ObjectCircle(ObjectCircle),
}

/// Drawable trait for Objects
pub trait Drawable {
    fn draw_object(&self);
}

/// Movable trait for Objects
pub trait Movable {
    fn move_object(&mut self, pos_x: f32, pos_y: f32);
}
