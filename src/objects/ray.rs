//! Ray object initialization and behaviors

use std::f32::consts::PI;

use crate::globals::{OBJC_MAX_RAY_COUNT, OBJD_RAY_COLOR, OBJD_RAY_WIDTH};

use macroquad::{
    color::Color, miniquad::gl::GL_TEXTURE_CUBE_MAP_POSITIVE_X, window::{screen_height, screen_width}
};

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
pub fn init_isotropic_rays(start_x: f32, start_y: f32) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(OBJC_MAX_RAY_COUNT as usize);

    for index in 0..OBJC_MAX_RAY_COUNT {
        let angle = (index as f32 / OBJC_MAX_RAY_COUNT as f32) * 2.0 * PI;

        rays.push(ObjectRay {
            start_x,
            start_y,
            end_x: start_x + angle.cos() * screen_width(),
            end_y: start_y + angle.sin() * screen_height(),
            thickness: OBJD_RAY_WIDTH,
            color: OBJD_RAY_COLOR,
        });
    }

    rays
}

pub fn init_collimated_rays(
    start_x: f32,
    start_y: f32,
    orientation: f32,
    collimated_beam_diameter: f32,
) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(OBJC_MAX_RAY_COUNT as usize);

    // the angle of each ray
    let cos_x = orientation.cos();
    let sin_y: f32 = orientation.sin();

    // the perpendicular angle of each ray
    let perp = (-sin_y, cos_x);

    // the spacing of each collimated ray
    let spacing: f32 = collimated_beam_diameter / (OBJC_MAX_RAY_COUNT - 1) as f32;

    for index in 0..OBJC_MAX_RAY_COUNT {
        let offset = (index - (OBJC_MAX_RAY_COUNT - 1) / 2) as f32 * spacing;
        let offset_x = offset * perp.0;
        let offset_y = offset * perp.1;

        rays.push(ObjectRay {
            start_x: start_x + offset_x,
            start_y: start_y + offset_y,
            end_x: start_x + offset_x + cos_x * screen_width(),
            end_y: start_y + offset_y + sin_y * screen_height(),
            thickness: OBJD_RAY_WIDTH,
            color: OBJD_RAY_COLOR,
        });
    }

    rays
}
