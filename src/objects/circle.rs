use super::behavior::{Drawable, Movable, RaytracerObjects};
use crate::globals::OBJC_COLLECTION;
use macroquad::prelude::*;

/// Structure for a circle general object
#[derive(Clone)]
pub struct ObjectCircle {
    pos_x: f32,
    pos_y: f32,
    color_fill: Color,
    radius: f32,
}

impl ObjectCircle {
    pub fn new(pos_x: f32, pos_y: f32, color_fill: Color, radius: f32) -> ObjectCircle {
        let new_circle: ObjectCircle = ObjectCircle {
            pos_x,
            pos_y,
            color_fill,
            radius,
        };

        // we don't draw the object here since we do it in the main loop
        OBJC_COLLECTION.lock().unwrap().push(RaytracerObjects::ObjectCircle(new_circle.clone()));
        
        new_circle
    }
}

/// Drawable Implementation for a Circle
impl Drawable for ObjectCircle {
    fn draw_object(&self) {
        draw_circle(self.pos_x, self.pos_y, self.radius, self.color_fill);
    }
}

/// Movable Implementation for a Circle
impl Movable for ObjectCircle {
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;

        // Might cause an error if an object without the Drawable trait moves
        self.draw_object();
    }
}
