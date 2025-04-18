//! Absorber objects initialization and behaviors
//!
//! This module provides light absorber implementation for the raytracer system.
//!
//! author:         Zhean Ganituen
//! last updated:   April 18, 2025

use super::behavior::*;
use super::circle::ObjectCircle;
use crate::OBJ_COLLECTION;

#[derive(Clone, Debug)]
pub enum Absorbers {
    AbsorberPerfect(AbsorberPerfect),
}

impl Drawable for Absorbers {
    fn draw_object(&self) {
        match self {
            Absorbers::AbsorberPerfect(obj) => obj.base_object.draw_object(),
        }
    }
}

impl Movable for Absorbers {
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        match self {
            Absorbers::AbsorberPerfect(obj) => obj.base_object.move_object(pos_x, pos_y),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AbsorberPerfect {
    pub base_object: ObjectCircle,
}

impl AbsorberPerfect {
    pub fn new(base_object: ObjectCircle) -> AbsorberPerfect {
        let new_object: AbsorberPerfect = AbsorberPerfect { base_object };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::Absorbers(Absorbers::AbsorberPerfect(
                new_object.clone(),
            )));

        new_object
    }
}
