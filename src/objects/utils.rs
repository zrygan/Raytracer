//! Utility functions for raytracer objects
//!
//! This module provides helper functions used throughout the raytracer for
//! calculations related to object positioning, ray transformations, and 
//! coordinate system operations.
//! 
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 16, 2025

/// Converts a vector (angle, start point) to a pair of points.
///
/// This function takes a vector defined by an angle and magnitude, and calculates
/// the end coordinates when that vector is applied from a given start point.
/// This is useful for ray calculations and object placement.
///
/// # Arguments
///
/// * `theta` - The angle in degrees (Macroquad uses degrees for angles)
/// * `magnitude` - The magnitude of the vector, typically (screen_width, screen_height)
/// * `start_point` - The initial point (x, y) from which the vector extends
///
/// # Returns
///
/// A tuple containing four values:
/// * The x-coordinate of the start point
/// * The y-coordinate of the start point
/// * The x-coordinate of the end point 
/// * The y-coordinate of the end point
///
/// # Example
///
/// ```
/// // Calculate end point for a ray starting at (400, 300) with 45 degree angle
/// let (start_x, start_y, end_x, end_y) = vec_to_coords(
///     45.0,
///     (screen_width(), screen_height()),
///     (400.0, 300.0)
/// );
/// ```
pub fn vec_to_coords(theta: f32, magnitude: (f32, f32), start_point: (f32, f32)) -> (f32, f32, f32, f32) {
    (
        start_point.0,
        start_point.1,
        start_point.0 + magnitude.0 * theta.cos(),
        start_point.1 + magnitude.0 * theta.sin(),
    )
}
