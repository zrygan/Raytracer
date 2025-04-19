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
    pub start_x: f32,
    /// Y coordinate of the ray's starting point
    pub start_y: f32,
    /// X coordinate of the ray's ending point
    pub end_x: f32,
    /// Y coordinate of the ray's ending point
    pub end_y: f32,
    /// Visual thickness of the ray when drawn
    pub thickness: f32,
    /// Color of the ray when drawn
    pub color: Color,
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
pub fn init_isotropic_rays(start_x: f32, start_y: f32, ray_count: i32) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(ray_count as usize);

    for index in 0..ray_count {
        // Calculate angle for each ray to distribute them evenly in a circle
        let angle = (index as f32 / ray_count as f32) * 2.0 * PI;

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
    ray_count: i32,
) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(ray_count as usize);

    // Calculate the direction vector components using the orientation angle
    let cos_x = orientation.cos();
    let sin_y: f32 = orientation.sin();

    // Calculate the perpendicular direction for ray spacing
    // (perpendicular to the main beam direction)
    let perp = (-sin_y, cos_x);

    // Calculate spacing between rays to achieve the desired beam diameter
    let spacing: f32 = collimated_beam_diameter / (ray_count - 1) as f32;

    for index in 0..ray_count {
        // Calculate offset from center for each ray
        // This creates evenly spaced rays centered on the emitter position
        let offset = (index - (ray_count - 1) / 2) as f32 * spacing;
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

/// Creates a collection of rays arranged in a spotlight pattern.
///
/// This function generates rays that originate from a single point and spread out
/// in a cone shape, similar to a spotlight or flashlight beam. The rays are evenly
/// distributed within the specified cone angle.
///
/// # Arguments
///
/// * `start_x` - X coordinate of the emitter's center point
/// * `start_y` - Y coordinate of the emitter's center point
/// * `orientation` - The central angle (in radians) at which the spotlight is pointing
/// * `spotlight_beam_angle` - The total angular spread of the spotlight cone (in radians)
///
/// # Returns
///
/// A vector of `ObjectRay`s arranged in a cone pattern from the given point
///
/// # Panics
///
/// Panics if `OBJC_MAX_RAY_COUNT` is less than 2, as at least two rays are needed to
/// define a spotlight beam (one at each edge of the cone).
pub fn init_spotlight_rays(
    start_x: f32,
    start_y: f32,
    orientation: f32,
    spotlight_beam_angle: f32,
    ray_count: i32,
) -> Vec<ObjectRay> {
    let mut rays: Vec<ObjectRay> = Vec::with_capacity(ray_count as usize);

    // Calculate the half-angle to evenly distribute rays on both sides of central orientation
    let half_angle = spotlight_beam_angle / 2.0;

    // Generate a linear range of angles from (orientation - half_angle) to (orientation + half_angle)
    // These will represent the directions of each ray in the spotlight cone
    let angles = linspace(
        orientation - half_angle,
        orientation + half_angle,
        ray_count,
    )
    .expect("Number of rays for spotlight must be at least 2.");

    // Create a ray for each angle in the spotlight cone
    for angle in angles {
        rays.push(ObjectRay::new(
            start_x,
            start_y,
            // Extend ray to screen edge in the direction of the angle
            // Note: Cosine gives x-component, and negative sine gives y-component (due to y-axis orientation)
            start_x + screen_width() * angle.cos(),
            start_y + screen_height() * (-1.0 * angle.sin()),
            OBJD_RAY_WIDTH,
            OBJD_RAY_COLOR,
        ));
    }

    rays
}
