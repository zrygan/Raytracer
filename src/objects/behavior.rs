//! Traits and common behaviors for raytracer objects
//!
//! This module defines the core traits that all renderable objects in the raytracer
//! must implement, as well as an enum for polymorphic handling of different object types.
//! It serves as the foundation for the object behavior system in the raytracer.
//!
//! # Core Components
//!
//! - `Drawable`: Trait for objects that can be visually rendered
//! - `Movable`: Trait for objects that can change position
//! - `RaytracerObjects`: Enum containing all possible object types in the system
//!
//! # Design Pattern
//!
//! This module implements a variant of the Strategy pattern, allowing different
//! objects to define their own drawing and movement behaviors while providing a
//! common interface for the rendering system.
//!
//! # Example
//!
//! ```rust
//! use crate::objects::behavior::{Drawable, Movable, RaytracerObjects};
//! use crate::objects::circle::ObjectCircle;
//!
//! // Create a circle object
//! let mut circle = ObjectCircle::new(100.0, 100.0, 50.0);
//!
//! // Move the circle using the Movable trait
//! circle.move_object(150.0, 150.0);
//!
//! // Draw the circle using the Drawable trait
//! circle.draw_object();
//!
//! // Wrap in a RaytracerObjects enum for polymorphic handling
//! let raytracer_obj = RaytracerObjects::ObjectCircle(circle);
//! ```
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 18, 2025

use super::absorber::Absorbers;
use super::circle::ObjectCircle;
use super::emitters::Emitters;

/// Enum that represents all possible object types in the raytracer.
///
/// This enum allows for polymorphic handling of different object types
/// in collections and the rendering system. Each variant wraps a concrete
/// object implementation.
///
/// Using this enum, the system can store heterogeneous collections of objects
/// while still providing type-specific behavior through pattern matching.
#[derive(Clone, Debug)]
pub enum RaytracerObjects {
    /// A simple circular shape object
    ObjectCircle(ObjectCircle),
    /// The enum for all emitter objects (light sources)
    Emitters(Emitters),
    /// The enum for all absorber objects (objects that block light)
    Absorbers(Absorbers),
}

impl RaytracerObjects {
    /// Gets the position of any RaytracerObject and returns it as a 2-tuple
    ///
    /// This method abstracts the process of retrieving an object's position
    /// regardless of its concrete type, providing a unified interface for
    /// position-based operations like collision detection or spatial queries.
    ///
    /// # Returns
    ///
    /// A tuple `(f32, f32)` containing the (x, y) coordinates of the object
    ///
    /// # Examples
    ///
    /// ```
    /// let circle = ObjectCircle::new(100.0, 100.0, 50.0);
    /// let obj = RaytracerObjects::ObjectCircle(circle);
    /// let (x, y) = obj.get_pos();
    /// assert_eq!(x, 100.0);
    /// assert_eq!(y, 100.0);
    /// ```
    pub fn get_pos(&self) -> (f32, f32) {
        match self {
            RaytracerObjects::ObjectCircle(object) => (object.pos_x, object.pos_y),
            RaytracerObjects::Emitters(emitter) => match emitter {
                Emitters::EmitterIsotropic(object) => {
                    (object.base_object.pos_x, object.base_object.pos_y)
                }
                Emitters::EmitterCollimated(object) => (
                    object.base_emitter.base_object.pos_x,
                    object.base_emitter.base_object.pos_y,
                ),
                Emitters::EmitterSpotlight(object) => (
                    object.base_emitter.base_object.pos_x,
                    object.base_emitter.base_object.pos_y,
                ),
            },
            RaytracerObjects::Absorbers(absorber) => match absorber {
                Absorbers::AbsorberPerfect(object) => {
                    (object.base_object.pos_x, object.base_object.pos_y)
                }
            },
        }
    }
}

/// Trait for objects that can be rendered to the screen.
///
/// All visible objects in the raytracer must implement this trait,
/// which provides the necessary method to render the object.
///
/// This trait is a core part of the rendering system, allowing the
/// main rendering loop to simply iterate through objects and call
/// `draw_object()` without needing to know the specific type details.
pub trait Drawable {
    /// Renders the object to the screen.
    ///
    /// Each implementing type will define its own rendering behavior
    /// appropriate to its visual representation.
    ///
    /// # Implementation Notes
    ///
    /// Implementors should:
    /// - Use the appropriate Macroquad drawing functions
    /// - Consider the object's properties (color, size, etc.)
    /// - Handle any transformation or special effects
    fn draw_object(&self);
}

/// Trait for objects that can be moved to a new position.
///
/// Provides functionality for interactive placement and animation
/// of objects within the raytracer environment.
///
/// This trait is essential for:
/// - User interaction (dragging objects)
/// - Animation systems
/// - Programmatic positioning
pub trait Movable {
    /// Moves the object to a new position.
    ///
    /// Updates the object's internal state to reflect its new position
    /// in the 2D coordinate space. The implementation should handle any
    /// side effects of the movement, such as updating dependent objects
    /// or recalculating derived properties.
    ///
    /// # Arguments
    ///
    /// * `pos_x` - The new x-coordinate position
    /// * `pos_y` - The new y-coordinate position
    ///
    /// # Examples
    ///
    /// ```
    /// let mut circle = ObjectCircle::new(0.0, 0.0, 50.0);
    /// circle.move_object(100.0, 200.0);
    /// assert_eq!(circle.pos_x, 100.0);
    /// assert_eq!(circle.pos_y, 200.0);
    /// ```
    fn move_object(&mut self, pos_x: f32, pos_y: f32);
}

pub trait VariableSize {
    fn change_radius(&mut self, factor: f32);
}
