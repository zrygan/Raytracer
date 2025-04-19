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
use std::sync::RwLock;

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
pub const WINDOW_USE_FRAME_RATE: bool = true;
pub const WINDOW_FRAME_RATE: f32 = 1. / 45.;

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
pub const MACROQUAD_SAMPLE_COUNT: i32 = 10; // MSAA sample count
pub const MACROQUAD_RESIZEABLE: bool = true;

/// Raytracer Object Collection
///
/// Thread-safe global collection of all objects in the raytracer scene.
/// Uses a mutex to allow safe mutation from different parts of the code.
pub static OBJ_COLLECTION: Lazy<RwLock<Vec<RaytracerObjects>>> =
    Lazy::new(|| RwLock::new(Vec::new()));

/// Raytracer Object Constants (starts with the OBJC_ prefix)
///
/// These constants define limitations for object counts to prevent performance issues
/// and memory overflow.
pub const OBJC_MAX_OBJ_COUNT: i32 = 100;
pub const OBJC_MAX_RAY_COUNT: i32 = 100;
pub const OBJC_MIN_RAY_COUNT: i32 = 3;
pub const OBJC_MOUSE_EPSILON: f32 = 5.0;

/// Raytracer Default Object Parameters (starts with OBJD_ prefix)
///
/// These constants define the default visual appearance and dimensions of
/// raytracer objects when created.
pub const OBJD_CIRCLE_RADIUS: f32 = 50.0;
pub const OBJD_CIRCLE_FILL: Color = CORNFLOWER_BLUE;
pub const OBJD_RAY_WIDTH: f32 = 1.0;
pub const OBJD_RAY_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const OBJD_RAY_COUNT: i32 = 32;
pub const OBJD_COLLIMATED_BEAM_DIAMETER: f32 = 2.0 * OBJD_CIRCLE_RADIUS;
pub const OBJD_COLLIMATED_ORIENTATION: f32 = 0.0; // in radians
pub const OBJD_SPOTLIGHT_BEAM_ANGLE: f32 = PI / 3.0; // in radians
pub const OBJD_SPOTLIGHT_ORIENTATION: f32 = 0.0; // in radians
pub const OBJD_ENLARGE_REDUCE_FACTOR: f32 = 5.;

/// Raytracer Keybinds (starts with KEYB_ prefix)
///
/// These constants map keyboard keys to specific actions in the raytracer,
/// making it easy to modify keybindings from a central location.
pub const KEYB_DELETE: KeyCode = KeyCode::Backspace;
pub const KEYB_SIMPLE_CIRCLE: KeyCode = KeyCode::O;
pub const KEYB_EMITTER_ISOTROPIC: KeyCode = KeyCode::I;
pub const KEYB_EMITTER_COLLIMATED: KeyCode = KeyCode::C;
pub const KEYB_EMITTER_SPOTLIGHT: KeyCode = KeyCode::S;
pub const KEYB_ABSORBER_PERFECT: KeyCode = KeyCode::P;
pub const KEYB_DEBUG_SHOW_ALL_OBJ: KeyCode = KeyCode::Backslash;

/// Raytracer Keybinds for Objects (starts with KEYB_RTC_ prefix)
///
/// These constants map keyboard keys to specific actions in raytracer when the
/// user is hovering on a Raytracer object
pub const KEYB_RTC_ENLARGE: KeyCode = KeyCode::Equal;
pub const KEYB_RTC_REDUCE: KeyCode = KeyCode::Minus;

/// Raytracer Keybinds for Emitters (starts with KEYB_EMM_ prefix)
///
/// These constants  map keyboard keys to specific actions in raytracer when the
/// user is hovering on a Emitters type object
pub const KEYB_EMM_ADD_RAYS: KeyCode = KeyCode::RightBracket;
pub const KEYB_EMM_REDUCE_RAYS: KeyCode = KeyCode::LeftBracket;
