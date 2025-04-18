//! Circle object initialization and behaviors
//!
//! This module defines the circle object type used throughout the raytracer.
//! Circle objects serve as both standalone entities and as the visual base
//! for other object types like emitters. The module provides implementation
//! of the core `Drawable` and `Movable` traits.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 16, 2025

use super::behavior::{Drawable, Movable, RaytracerObjects};
use crate::globals::*;

use macroquad::prelude::*;

/// Represents a basic circle object in the raytracer.
///
/// Circle objects are the fundamental building blocks for many raytracer
/// elements including barriers, emitter bases, and interactive elements.
/// They have position, color, and size properties.
#[derive(Clone, Debug)]
pub struct ObjectCircle {
    /// X-coordinate of the circle's center position
    pub pos_x: f32,
    /// Y-coordinate of the circle's center position
    pub pos_y: f32,
    /// Fill color of the circle when rendered
    pub color_fill: Color,
    /// Radius of the circle in pixels
    pub radius: f32,
}

impl ObjectCircle {
    /// Creates a new circle object with the specified properties.
    ///
    /// This constructor also adds the newly created circle to the global
    /// object collection for tracking in the raytracer system.
    ///
    /// # Arguments
    ///
    /// * `pos_x` - X-coordinate of the circle's center position
    /// * `pos_y` - Y-coordinate of the circle's center position
    /// * `color_fill` - Fill color of the circle when rendered
    /// * `radius` - Radius of the circle in pixels
    ///
    /// # Returns
    ///
    /// A new `ObjectCircle` instance with the specified parameters
    pub fn new(pos_x: f32, pos_y: f32, color_fill: Color, radius: f32) -> ObjectCircle {
        ObjectCircle {
            pos_x,
            pos_y,
            color_fill,
            radius,
        }
    }

    pub fn new_and_add(pos_x: f32, pos_y: f32, color_fill: Color, radius: f32) -> ObjectCircle {
        let new_circle: ObjectCircle = ObjectCircle::new(pos_x, pos_y, color_fill, radius);

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::ObjectCircle(new_circle.clone()));

        new_circle
    }
}

/// Drawable Implementation for a Circle
impl Drawable for ObjectCircle {
    /// Renders the circle to the screen.
    ///
    /// Uses the macroquad rendering function to draw a filled circle
    /// at the object's position with its color and radius.
    fn draw_object(&self) {
        draw_circle(self.pos_x, self.pos_y, self.radius, self.color_fill);
    }
}

/// Movable Implementation for a Circle
impl Movable for ObjectCircle {
    /// Moves the circle to a new position.
    ///
    /// Updates the circle's coordinates and redraws it at the new location.
    ///
    /// # Arguments
    ///
    /// * `pos_x` - The new x-coordinate position
    /// * `pos_y` - The new y-coordinate position
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;

        // Might cause an error if an object without the Drawable trait moves
        // TODO: @zrygan
        self.draw_object();
    }
}
