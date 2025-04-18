//! Helper utilities for the Raytracer application
//!
//! This module organizes various utility functions that support the core raytracer
//! functionality. It contains submodules that handle object management, mathematical
//! operations, and user actions.
//!
//! # Modules
//!
//! * `action_utils` - Functions for user interactions with objects (selecting, removing, etc.)
//! * `object_utils` - Utility functions for object creation, manipulation and mathematical operations
//!
//! # Usage
//!
//! These helper modules are typically used by the main application logic to perform
//! common operations without cluttering the core simulation code.
//!
//! ```rust
//! use crate::helpers::action_utils::object_at_cursor;
//! use crate::helpers::object_utils::linspace;
//!
//! // Find an object at the current cursor position
//! if let Some(index) = object_at_cursor(mouse_x, mouse_y) {
//!     // Do something with the object
//! }
//!
//! // Generate a sequence of evenly spaced values
//! let angles = linspace(0.0, std::f32::consts::PI, 10)
//!     .expect("Invalid parameters for linspace");
//! ```
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 18, 2025

/// Utilities for interacting with objects in the scene
pub mod action_utils;

/// Mathematical and object creation/manipulation utilities
pub mod object_utils;
