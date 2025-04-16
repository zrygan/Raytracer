//! Utility functions for RaytracerObject

/// Converts a vector (angle, start point) to a pair of points.
/// Returns a four tuple, the first two elements being the xy coordinates of the
/// first point, and the other two the xy coordinates of the second point
/// theta must be in degrees (since Macroquad uses degrees)
/// magnitude is the screen size (screen_width, screen_height)
pub fn vec_to_coords(theta: f32, magnitude: (f32, f32), start_point: (f32, f32)) -> (f32, f32, f32, f32) {
    (
        start_point.0,
        start_point.1,
        start_point.0 + magnitude.0 * theta.cos(),
        start_point.1 + magnitude.0 * theta.sin(),
    )
}
