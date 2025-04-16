//! Ray object initializatin and behaviors

use crate::globals::{OBJC_MAX_RAY_COUNT, OBJD_RAY_COLOR, OBJD_RAY_WIDTH};

use macroquad::color::Color;

#[derive(Clone)]
pub struct ObjectRay {
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub thickness: f32,
    pub color: Color,
}

impl ObjectRay {
    pub fn new(
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
    ) -> ObjectRay {
        let ray: ObjectRay = ObjectRay {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color,
        };

        ray
    }
}

/// Function to create rays for a point emitter
pub fn init_rays_for_point(start_x: f32, start_y: f32) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(OBJC_MAX_RAY_COUNT as usize);

    for index in 0..OBJC_MAX_RAY_COUNT - 1 {
        let angle = (index * 360) as f32 / OBJC_MAX_RAY_COUNT as f32;

        rays.push(ObjectRay {
            start_x,
            start_y,
            end_x: angle.cos(),
            end_y: angle.sin(),
            thickness: OBJD_RAY_WIDTH,
            color: OBJD_RAY_COLOR,
        });
    }
    
    rays
}
