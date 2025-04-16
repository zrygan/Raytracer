//! Emitter objects initalization and behaviors
use macroquad::shapes::{draw_circle, draw_line};

use super::behavior::{Drawable, Movable, RaytracerObjects};
use super::circle::ObjectCircle;
use super::ray::ObjectRay;
use crate::OBJ_COLLECTION;

pub enum Emitters {
    EmitterPoint(EmitterPoint),
}

#[derive(Clone)]
pub struct EmitterPoint {
    pub base_object: ObjectCircle,
    pub rays: Vec<ObjectRay>,
}

impl EmitterPoint {
    pub fn new(base_object: ObjectCircle, rays: Vec<ObjectRay>) -> EmitterPoint {
        // if rays.len() > OBJC_MAX_RAY_COUNT as usize {
        //     panic!("Raytracer: EmitterPoint has too many rays. See OBJC_MAX_RAY_COUNT in globals.")
        // }

        let new_emitter: EmitterPoint = EmitterPoint { base_object, rays };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::EmitterPoint(new_emitter.clone()));

        new_emitter
    }
}

impl Drawable for EmitterPoint {
    fn draw_object(&self) {
        draw_circle(
            self.base_object.pos_x,
            self.base_object.pos_y,
            self.base_object.radius,
            self.base_object.color_fill,
        );

        for ray in self.rays.clone() {
            draw_line(
                ray.start_x,
                ray.start_y,
                ray.end_x,
                ray.end_y,
                ray.thickness,
                ray.color,
            );
        }
    }
}

impl Movable for EmitterPoint {
    #[warn(unused_variables)]
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {}
}
