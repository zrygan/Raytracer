//! Contains all global variables for the Raytracer project
//!
//! This module centralizes configuration constants and shared resources for the raytracer.
//! It includes application metadata, window settings, visual defaults, keybindings,
//! and object limitations used throughout the application.

use crate::objects::behavior::RaytracerObjects;
use macroquad::input::KeyCode::{self};
use macroquad::prelude::Color;
use once_cell::sync::Lazy;
use std::f32::consts::PI;
use std::sync::Mutex;

/// App Information (starts with the APP_ prefix)
///
/// These constants contain metadata about the application itself that may be
/// displayed in window titles, about screens, or for identifying the application.
pub const APP_NAME: &str = "Raytracer";
pub const APP_VERSION: &str = "v0.rp";
pub const APP_AUTHOR: &str = "Zhean Ganituen";
pub const APP_GITHUB: &str = "https://github.com/zrygan/raytracer";
// pub const APP_ICON: Icon;

/// Window Settings (starts with the WINDOW_ prefix)
///
/// These constants define the visual appearance and dimensions of the application window.
pub const WINDOW_HEIGHT: i32 = 800;
pub const WINDOW_WIDTH: i32 = 600;
pub const WINDOW_BG_COLOR: Color = Color::new(0.00, 0.00, 0.00, 1.00); // Black

/// Standard Colors
///
/// Common colors used throughout the application for consistent visual styling.
pub const CORNFLOWER_BLUE: Color = Color::new(0.39, 0.58, 0.92, 1.00);

/// Macroquad Set Up (starts with the MACROQUAD_ prefix)
///
/// These constants configure the Macroquad rendering engine behavior including
/// display quality options and window properties.
pub const MACROQUAD_HIGH_DPI: bool = false;
pub const MACROQUAD_FULLSCREEN: bool = false;
pub const MACROQUAD_SAMPLE_COUNT: i32 = 0; // MSAA sample count
pub const MACROQUAD_RESIZEABLE: bool = false;

/// Raytracer Object Collection
///
/// Thread-safe global collection of all objects in the raytracer scene.
/// Uses a mutex to allow safe mutation from different parts of the code.
pub static OBJ_COLLECTION: Lazy<Mutex<Vec<RaytracerObjects>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

/// Raytracer Object Constants (starts with the OBJC_ prefix)
///
/// These constants define limitations for object counts to prevent performance issues
/// and memory overflow.
pub const OBJC_MAX_OBJ_COUNT: i32 = 100;
pub const OBJC_MAX_RAY_COUNT: i32 = 25;

/// Raytracer Default Object Parameters (starts with OBJD_ prefix)
///
/// These constants define the default visual appearance and dimensions of
/// raytracer objects when created.
pub const OBJD_CIRCLE_RADIUS: f32 = 50.0;
pub const OBJD_CIRCLE_FILL: Color = CORNFLOWER_BLUE;
pub const OBJD_RAY_WIDTH: f32 = 1.0;
pub const OBJD_RAY_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const OBJD_COLLIMATED_BEAM_DIAMETER: f32 = 2.0 * OBJD_CIRCLE_RADIUS;
pub const OBJD_COLLIMATED_ORIENTATION: f32 = 0.0; // in radians
pub const OBJD_SPOTLIGHT_BEAM_ANGLE: f32 = PI / 3.0; // in radians
pub const OBJD_SPOTLIGHT_ORIENTATION: f32 = 0.0; // in radians

/// Raytracer Keybinds (starts with KEYB_ prefix)
///
/// These constants map keyboard keys to specific actions in the raytracer,
/// making it easy to modify keybindings from a central location.
pub const KEYB_SIMPLE_CIRCLE: KeyCode = KeyCode::Period;
pub const KEYB_EMITTER_ISOTROPIC: KeyCode = KeyCode::I;
pub const KEYB_EMITTER_COLLIMATED: KeyCode = KeyCode::C;
pub const KEYB_EMITTER_SPOTLIGHT: KeyCode = KeyCode::S;
