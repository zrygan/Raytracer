//! Traits and common behaviors for raytracer objects
//!
//! This module defines the core traits that all renderable objects in the raytracer
//! must implement, as well as an enum for polymorphic handling of different object types.
//! It serves as the foundation for the object behavior system in the raytracer.
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
#[derive(Clone, Debug)]
pub enum RaytracerObjects {
    /// A simple circular shape object
    ObjectCircle(ObjectCircle),
    /// The enum for all emitter objects
    Emitters(Emitters),
    /// The enum for all absorber objects
    Absorbers(Absorbers),
}

impl RaytracerObjects {
    /// Gets the position of any RaytracerObject and returns it as a 2-tuple
    ///
    /// # Arguments
    ///
    /// *`&self` - pointer to the RaytracerObject
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
pub trait Drawable {
    /// Renders the object to the screen.
    ///
    /// Each implementing type will define its own rendering behavior
    /// appropriate to its visual representation.
    fn draw_object(&self);
}

/// Trait for objects that can be moved to a new position.
///
/// Provides functionality for interactive placement and animation
/// of objects within the raytracer environment.
pub trait Movable {
    /// Moves the object to a new position.
    ///
    /// # Arguments
    ///
    /// * `pos_x` - The new x-coordinate position
    /// * `pos_y` - The new y-coordinate position
    fn move_object(&mut self, pos_x: f32, pos_y: f32);
}
