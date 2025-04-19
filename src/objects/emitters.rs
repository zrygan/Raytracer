//! Emitter objects initialization and behaviors
//!
//! This module provides light emitter implementations for the raytracer system.
//! It defines three types of emitters: isotropic (radiating in all directions),
//! collimated (parallel rays, like a laser), and spotlight (cone-shaped beam).
//!
//! author:         Zhean Ganituen
//! last updated:   April 18, 2025

use macroquad::shapes::draw_circle;

use crate::globals::{OBJC_MAX_RAY_COUNT, OBJC_MIN_RAY_COUNT, OBJD_RAY_COUNT};

use super::behavior::{Drawable, Movable, VariableSize};
use super::circle::ObjectCircle;
use super::ray::{ObjectRay, init_collimated_rays, init_isotropic_rays, init_spotlight_rays};

/// Enumeration of all emitter types supported by the raytracer.
///
/// This enum allows different emitter types to be treated polymorphically
/// in the rendering and physics systems.
#[derive(Clone, Debug)]
pub enum Emitters {
    /// Standard isotropic emitter that radiates light in all directions
    EmitterIsotropic(EmitterIsotropic),
    /// Directional emitter that produces parallel rays
    EmitterCollimated(EmitterCollimated),
    /// Spotlight emitter that produces a cone-shaped beam
    EmitterSpotlight(EmitterSpotlight),
}

pub trait VariableRays {
    fn change_rays_count(&mut self, change_rays: i32);
}

impl Drawable for Emitters {
    /// Draws the emitter on the screen.
    ///
    /// Delegates to the appropriate draw_object method based on the emitter type.
    fn draw_object(&self) {
        match self {
            Emitters::EmitterIsotropic(e) => e.draw_object(),
            Emitters::EmitterCollimated(e) => e.base_emitter.draw_object(),
            Emitters::EmitterSpotlight(e) => e.base_emitter.draw_object(),
        }
    }
}

impl Movable for Emitters {
    /// Moves the emitter to a new position.
    ///
    /// Delegates to the appropriate move_object method based on the emitter type.
    ///
    /// # Arguments
    ///
    /// * `pos_x` - The new x-coordinate position
    /// * `pos_y` - The new y-coordinate position
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        match self {
            Emitters::EmitterIsotropic(obj) => {
                let ray_count = if obj.rays.is_empty() {
                    OBJD_RAY_COUNT
                } else {
                    obj.rays.len() as i32
                };

                obj.base_object.pos_x = pos_x;
                obj.base_object.pos_y = pos_y;
                obj.rays = init_isotropic_rays(pos_x, pos_y, ray_count);
            }
            Emitters::EmitterCollimated(obj) => {
                let ray_count = if obj.base_emitter.rays.is_empty() {
                    OBJD_RAY_COUNT
                } else {
                    obj.base_emitter.rays.len() as i32
                };

                obj.base_emitter.base_object.pos_x = pos_x;
                obj.base_emitter.base_object.pos_y = pos_y;
                obj.base_emitter.rays = init_collimated_rays(
                    pos_x,
                    pos_y,
                    obj.orientation,
                    obj.collimated_beam_diameter,
                    ray_count,
                );
            }
            Emitters::EmitterSpotlight(obj) => {
                let ray_count = if obj.base_emitter.rays.is_empty() {
                    OBJD_RAY_COUNT
                } else {
                    obj.base_emitter.rays.len() as i32
                };

                obj.base_emitter.base_object.pos_x = pos_x;
                obj.base_emitter.base_object.pos_y = pos_y;
                obj.base_emitter.rays = init_spotlight_rays(
                    pos_x,
                    pos_y,
                    obj.orientation,
                    obj.spotlight_beam_angle,
                    ray_count,
                );
            }
        }
    }
}

impl VariableSize for Emitters {
    fn change_radius(&mut self, factor: f32) {
        match self {
            Emitters::EmitterIsotropic(obj) => {
                if obj.base_object.radius >= 30. && factor < 0. {
                    obj.base_object.radius += factor
                } else if factor > 0. {
                    obj.base_object.radius += factor;
                }
            }
            Emitters::EmitterCollimated(obj) => {
                if obj.base_emitter.base_object.radius >= 30. && factor < 0. {
                    obj.base_emitter.base_object.radius += factor
                } else if factor > 0. {
                    obj.base_emitter.base_object.radius += factor;
                }
            }
            Emitters::EmitterSpotlight(obj) => {
                if obj.base_emitter.base_object.radius >= 30. && factor < 0. {
                    obj.base_emitter.base_object.radius += factor
                } else if factor > 0. {
                    obj.base_emitter.base_object.radius += factor;
                }
            }
        }
    }

    fn get_radius(&self) -> f32 {
        match self {
            Emitters::EmitterIsotropic(obj) => return obj.base_object.radius,

            Emitters::EmitterCollimated(obj) => return obj.base_emitter.base_object.radius,

            Emitters::EmitterSpotlight(obj) => return obj.base_emitter.base_object.radius,
        }
    }
}

impl VariableRays for Emitters {
    fn change_rays_count(&mut self, change_rays: i32) {
        fn check_rays_range(ray_count: i32, change_rays: i32) {
            if ray_count + change_rays > OBJC_MAX_RAY_COUNT {
                eprintln!(
                    "Raytracer ~Err. Added too many rays, more than OBJC_MAX_RAY_COUNT. Program will still run, but may become unstable since there are too many rays."
                );
            } else if ray_count + change_rays < OBJC_MIN_RAY_COUNT {
                eprintln!(
                    "Raytracer ~Err. Cannot reduce below minimum ray count of {}. Operation ignored.",
                    OBJC_MIN_RAY_COUNT
                );
            }
        }

        match self {
            Emitters::EmitterIsotropic(obj) => {
                let ray_count = obj.rays.len() as i32;

                // Only proceed if we won't go below the minimum
                if ray_count + change_rays >= OBJC_MIN_RAY_COUNT {
                    obj.rays = init_isotropic_rays(
                        obj.base_object.pos_x,
                        obj.base_object.pos_y,
                        ray_count + change_rays,
                    );
                    check_rays_range(ray_count, change_rays);
                } else {
                    eprintln!(
                        "Raytracer ~Err. Cannot reduce below minimum ray count of {}.",
                        OBJC_MIN_RAY_COUNT
                    );
                }
            }
            Emitters::EmitterCollimated(obj) => {
                let ray_count = obj.base_emitter.rays.len() as i32;

                // Only proceed if we won't go below the minimum
                if ray_count + change_rays >= OBJC_MIN_RAY_COUNT {
                    obj.base_emitter.rays = init_collimated_rays(
                        obj.base_emitter.base_object.pos_x,
                        obj.base_emitter.base_object.pos_y,
                        obj.orientation,
                        obj.collimated_beam_diameter,
                        ray_count + change_rays,
                    );
                    check_rays_range(ray_count, change_rays);
                } else {
                    eprintln!(
                        "Raytracer ~Err. Cannot reduce below minimum ray count of {}.",
                        OBJC_MIN_RAY_COUNT
                    );
                }
            }
            Emitters::EmitterSpotlight(obj) => {
                let ray_count = obj.base_emitter.rays.len() as i32;

                // Only proceed if we won't go below the minimum
                if ray_count + change_rays >= OBJC_MIN_RAY_COUNT {
                    obj.base_emitter.rays = init_spotlight_rays(
                        obj.base_emitter.base_object.pos_x,
                        obj.base_emitter.base_object.pos_y,
                        obj.orientation,
                        obj.spotlight_beam_angle,
                        ray_count + change_rays,
                    );
                    check_rays_range(ray_count, change_rays);
                } else {
                    eprintln!(
                        "Raytracer ~Err. Cannot reduce below minimum ray count of {}.",
                        OBJC_MIN_RAY_COUNT
                    );
                }
            }
        }
    }
}

/// Represents a standard isotropic light emitter.
///
/// This emitter radiates light rays in all directions from a central point,
/// similar to a point light source in real-world physics.
#[derive(Clone, Debug)]
pub struct EmitterIsotropic {
    /// The physical representation of the emitter (position, size, color)
    pub base_object: ObjectCircle,
    /// Collection of light rays emanating from this emitter
    pub rays: Vec<ObjectRay>,
}

impl EmitterIsotropic {
    /// Creates a new isotropic emitter with the given properties.
    ///
    /// Adds the newly created emitter to the global object collection
    /// for tracking in the raytracer system.
    ///
    /// # Arguments
    ///
    /// * `base_object` - The physical properties of the emitter
    /// * `rays` - Collection of rays to be emitted from this source
    ///
    /// # Returns
    ///
    /// A new `EmitterIsotropic` instance with the specified parameters
    pub fn new(base_object: ObjectCircle, rays: Vec<ObjectRay>) -> Self {
        EmitterIsotropic { base_object, rays }
    }
}

impl Drawable for EmitterIsotropic {
    /// Draws the isotropic emitter and its rays on the screen.
    ///
    /// Renders the emitter as a colored circle and draws all of its
    /// associated light rays emanating from it.
    fn draw_object(&self) {
        // Draw the emitter's physical representation (a circle)
        draw_circle(
            self.base_object.pos_x,
            self.base_object.pos_y,
            self.base_object.radius,
            self.base_object.color_fill,
        );

        // Draw all the light rays associated with this emitter
        for ray in &self.rays {
            ray.draw_object();
        }
    }
}

/// Represents a collimated (parallel rays) light emitter.
///
/// This emitter produces rays that travel in parallel, similar to a laser beam
/// in real-world physics. It has an orientation and beam diameter.
#[derive(Clone, Debug)]
pub struct EmitterCollimated {
    /// The underlying emitter providing basic functionality
    pub base_emitter: EmitterIsotropic,
    /// The angle (in radians) at which rays are emitted
    pub orientation: f32,
    /// The width of the beam of parallel rays
    pub collimated_beam_diameter: f32,
}

impl EmitterCollimated {
    /// Creates a new collimated emitter with the specified properties.
    ///
    /// Adds the newly created emitter to the global object collection
    /// for tracking in the raytracer system.
    ///
    /// # Arguments
    ///
    /// * `base_emitter` - The physical properties of the emitter
    /// * `rays` - Collection of rays to be emitted from this source
    /// * `orientation` - The angle (in radians) at which rays are emitted
    /// * `collimated_beam_diameter` - The width of the beam of parallel rays
    ///
    /// # Returns
    ///
    /// A new `EmitterCollimated` instance with the specified parameters
    pub fn new(
        base_object: ObjectCircle,
        rays: Vec<ObjectRay>,
        orientation: f32,
        collimated_beam_diameter: f32,
    ) -> Self {
        EmitterCollimated {
            base_emitter: EmitterIsotropic { base_object, rays },
            orientation,
            collimated_beam_diameter,
        }
    }
}

/// Represents a spotlight emitter.
///
/// This emitter produces rays within a specific angular range, defined by the
/// `spotlight_beam_angle`. The rays are distributed within this angle, centered
/// around the `orientation` angle, creating a cone-like light effect.
#[derive(Clone, Debug)]
pub struct EmitterSpotlight {
    /// The underlying emitter providing basic functionality
    pub base_emitter: EmitterIsotropic,
    /// The central angle (in radians) at which the spotlight is directed
    pub orientation: f32,
    /// The angular range (in radians) within which rays are emitted
    pub spotlight_beam_angle: f32,
}

impl EmitterSpotlight {
    /// Creates a new spotlight emitter with the specified properties.
    ///
    /// Adds the newly created emitter to the global object collection
    /// for tracking in the raytracer system.
    ///
    /// # Arguments
    ///
    /// * `base_object` - The physical properties of the emitter
    /// * `rays` - Collection of rays to be emitted from this source
    /// * `orientation` - The central angle (in radians) at which the spotlight is directed
    /// * `spotlight_beam_angle` - The angular range (in radians) within which rays are emitted,
    ///   forming a cone of light. A smaller angle creates a narrower beam, while a larger angle
    ///   creates a wider beam.
    ///
    /// # Returns
    ///
    /// A new `EmitterSpotlight` instance with the specified parameters
    pub fn new(
        base_object: ObjectCircle,
        rays: Vec<ObjectRay>,
        orientation: f32,
        spotlight_beam_angle: f32,
    ) -> Self {
        EmitterSpotlight {
            base_emitter: EmitterIsotropic { base_object, rays },
            orientation,
            spotlight_beam_angle,
        }
    }
}
