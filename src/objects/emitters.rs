//! Emitter objects initalization and behaviors

use macroquad::shapes::{draw_circle, draw_line};

use super::behavior::{Drawable, Movable, RaytracerObjects};
use super::circle::ObjectCircle;
use super::ray::ObjectRay;
use crate::OBJ_COLLECTION;

pub enum Emitters {
    Emitter(Emitter),
    EmitterCollimated(EmitterCollimated),
}

impl Drawable for Emitters {
    fn draw_object(&self) {
        match self {
            Emitters::Emitter(e) => e.draw_object(),
            Emitters::EmitterCollimated(e) => e.base_emitter.draw_object(),
        }
    }
}

impl Movable for Emitters {
    // #[warn(unused_variables)]
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        match self {
            Emitters::Emitter(e) => e.move_object(pos_x, pos_y),
            Emitters::EmitterCollimated(e) => e.base_emitter.move_object(pos_x, pos_y),
        }
    }
}

impl Drawable for Emitter {
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

impl Movable for Emitter {
    // #[warn(unused_variables)]
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        println!("{}{}", pos_x, pos_y)
    }
}

#[derive(Clone)]
pub struct Emitter {
    pub base_object: ObjectCircle,
    pub rays: Vec<ObjectRay>,
}

impl Emitter {
    pub fn new(base_object: ObjectCircle, rays: Vec<ObjectRay>) -> Emitter {
        // if rays.len() > OBJC_MAX_RAY_COUNT as usize {
        //     panic!("Raytracer: Emitter has too many rays. See OBJC_MAX_RAY_COUNT in globals.")
        // }

        let new_emitter: Emitter = Emitter { base_object, rays };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::Emitter(new_emitter.clone()));

        new_emitter
    }
}

#[derive(Clone)]
pub struct EmitterCollimated {
    pub base_emitter: Emitter,
    pub orientation: f32,
    pub collimated_beam_diameter: f32,
}

impl EmitterCollimated {
    pub fn new(
        base_object: ObjectCircle,
        rays: Vec<ObjectRay>,
        orientation: f32,
        collimated_beam_diameter: f32,
    ) -> EmitterCollimated {
        let new_emitter = EmitterCollimated {
            base_emitter: Emitter { base_object, rays },
            orientation,
            collimated_beam_diameter,
        };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::EmitterCollimated(new_emitter.clone()));

        new_emitter
    }
}
