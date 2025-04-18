//! Object module declarations for the raytracer
//!
//! This module organizes the various components that make up the raytracer's
//! object system. It includes:
//!
//! - `absorbers:` Light absorber implementations
//! - `behavior`: Core traits and enums for object behaviors
//! - `circle`: Basic circle objects that serve as building blocks
//! - `emitters`: Light emitter implementations (isotropic and collimated)
//! - `ray`: Ray objects that represent light paths
//! - `utils`: Utility functions for object manipulations
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 16, 2025

pub mod absorber;
pub mod behavior;
pub mod circle;
pub mod emitters;
pub mod ray;
