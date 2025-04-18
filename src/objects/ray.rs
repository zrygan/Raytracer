//! Ray object initialization and behaviors
//!
//! This module contains the definition of light rays used in the raytracer,
//! including both the base `ObjectRay` struct and helper functions to create
//! different ray patterns.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 17, 2025

use std::f32::consts::PI;

use super::behavior::Drawable;
use crate::globals::{OBJC_MAX_RAY_COUNT, OBJD_RAY_COLOR, OBJD_RAY_WIDTH};
use crate::helpers::object_utils::linspace;

use macroquad::{
    color::Color,
    shapes::draw_line,
    window::{screen_height, screen_width},
};

/// Represents a single light ray in the raytracer.
///
/// A ray has a starting point, ending point, visual properties (thickness and color),
/// and is used by emitter objects to simulate light propagation.
#[derive(Clone, Debug)]
pub struct ObjectRay {
    /// X coordinate of the ray's starting point
    start_x: f32,
    /// Y coordinate of the ray's starting point
    start_y: f32,
    /// X coordinate of the ray's ending point
    end_x: f32,
    /// Y coordinate of the ray's ending point
    end_y: f32,
    /// Visual thickness of the ray when drawn
    thickness: f32,
    /// Color of the ray when drawn
    color: Color,
}

impl ObjectRay {
    /// Creates a new ray with specified parameters.
    ///
    /// This constructor provides a clean way to create an ObjectRay instance
    /// with all necessary properties.
    ///
    /// # Arguments
    ///
    /// * `start_x` - X coordinate of the ray's starting point
    /// * `start_y` - Y coordinate of the ray's starting point
    /// * `end_x` - X coordinate of the ray's ending point
    /// * `end_y` - Y coordinate of the ray's ending point
    /// * `thickness` - Visual thickness of the ray when drawn
    /// * `color` - Color of the ray when drawn
    ///
    /// # Returns
    ///
    /// A new `ObjectRay` instance with the specified parameters
    pub fn new(
        start_x: f32,
        start_y: f32,
        end_x: f32,
        end_y: f32,
        thickness: f32,
        color: Color,
    ) -> Self {
        Self {
            start_x,
            start_y,
            end_x,
            end_y,
            thickness,
            color,
        }
    }
}

impl Drawable for ObjectRay {
    fn draw_object(&self) {
        draw_line(
            self.start_x,
            self.start_y,
            self.end_x,
            self.end_y,
            self.thickness,
            self.color,
        );
    }
}

/// Creates a collection of rays arranged in an isotropic (point source) pattern.
///
/// This function generates rays that emanate from a central point in all directions,
/// similar to a point light source. The rays are evenly distributed around 360 degrees.
///
/// # Arguments
///
/// * `start_x` - X coordinate of the emitter's center point
/// * `start_y` - Y coordinate of the emitter's center point
///
/// # Returns
///
/// A vector of `ObjectRay`s arranged in a circular pattern from the given point
pub fn init_isotropic_rays(start_x: f32, start_y: f32) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(OBJC_MAX_RAY_COUNT as usize);

    for index in 0..OBJC_MAX_RAY_COUNT {
        // Calculate angle for each ray to distribute them evenly in a circle
        let angle = (index as f32 / OBJC_MAX_RAY_COUNT as f32) * 2.0 * PI;

        rays.push(ObjectRay::new(
            start_x,
            start_y,
            start_x + angle.cos() * screen_width(),
            start_y + angle.sin() * screen_height(),
            OBJD_RAY_WIDTH,
            OBJD_RAY_COLOR,
        ));
    }

    rays
}

/// Creates a collection of rays arranged in a parallel (collimated) pattern.
///
/// This function generates rays that are parallel to each other, similar to a laser beam
/// or directional light. The rays are evenly spaced across a line perpendicular to their direction.
///
/// # Arguments
///
/// * `start_x` - X coordinate of the emitter's center point
/// * `start_y` - Y coordinate of the emitter's center point
/// * `orientation` - The angle (in radians) at which the rays should point
/// * `collimated_beam_diameter` - Width of the beam (perpendicular to ray direction)
///
/// # Returns
///
/// A vector of `ObjectRay`s arranged in a parallel pattern from the given point
pub fn init_collimated_rays(
    start_x: f32,
    start_y: f32,
    orientation: f32,
    collimated_beam_diameter: f32,
) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(OBJC_MAX_RAY_COUNT as usize);

    // Calculate the direction vector components using the orientation angle
    let cos_x = orientation.cos();
    let sin_y: f32 = orientation.sin();

    // Calculate the perpendicular direction for ray spacing
    // (perpendicular to the main beam direction)
    let perp = (-sin_y, cos_x);

    // Calculate spacing between rays to achieve the desired beam diameter
    let spacing: f32 = collimated_beam_diameter / (OBJC_MAX_RAY_COUNT - 1) as f32;

    for index in 0..OBJC_MAX_RAY_COUNT {
        // Calculate offset from center for each ray
        // This creates evenly spaced rays centered on the emitter position
        let offset = (index - (OBJC_MAX_RAY_COUNT - 1) / 2) as f32 * spacing;
        let offset_x = offset * perp.0;
        let offset_y = offset * perp.1;

        rays.push(ObjectRay::new(
            // Apply offset to create parallel rays
            start_x + offset_x,
            start_y + offset_y,
            // Extend ray to screen edge in the direction of orientation
            start_x + offset_x + cos_x * screen_width(),
            start_y + offset_y + sin_y * screen_height(),
            OBJD_RAY_WIDTH,
            OBJD_RAY_COLOR,
        ));
    }

    rays
}

pub fn init_spotlight_rays(
    start_x: f32,
    start_y: f32,
    orientation: f32,
    spotlight_beam_angle: f32,
) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(OBJC_MAX_RAY_COUNT as usize);

    // generate angles that are linearly spaced
    let half_angle = spotlight_beam_angle / 2.0;
    let angles = linspace(
        orientation - half_angle,
        orientation + half_angle,
        OBJC_MAX_RAY_COUNT,
    )
    .expect("Number of rays for spotlight must be at least 2.");

    for angle in angles {
        rays.push(ObjectRay::new(
            start_x,
            start_y,
            start_x + screen_width() * angle.cos(),
            start_y + screen_height() * (-1.0 * angle.sin()),
            OBJD_RAY_WIDTH,
            OBJD_RAY_COLOR,
        ));
    }

    rays
}
