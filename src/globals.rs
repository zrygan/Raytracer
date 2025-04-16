//! Contains all global variables for the Raytracer project

use crate::objects::behavior::RaytracerObjects;
use macroquad::input::KeyCode::{self};
use macroquad::prelude::Color;
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// App Information (starts with the APP_ prefix)
pub const APP_NAME: &str = "Raytracer";
pub const APP_VERSION: &str = "v0.rp";
pub const APP_AUTHOR: &str = "Zhean Ganituen";
pub const APP_GITHUB: &str = "https://github.com/zrygan/raytracer";
// pub const APP_ICON: Icon;

/// Window Settings (starts with the WINDOW_ prefix)
pub const WINDOW_HEIGHT: i32 = 800;
pub const WINDOW_WIDTH: i32 = 600;
pub const WINDOW_BG_COLOR: Color = Color::new(0.00, 0.00, 0.00, 1.00);

// Colors
pub const CORNFLOWER_BLUE: Color = Color::new(1.00, 0.34, 0.20, 1.00);

/// Macroquad Set Up (starts with the MACROQUAD_ prefix)
pub const MACROQUAD_HIGH_DPI: bool = false;
pub const MACROQUAD_FULLSCREEN: bool = false;
pub const MACROQUAD_SAMPLE_COUNT: i32 = 0; // MSAA sample count
pub const MACROQUAD_RESIZEABLE: bool = false;

/// Raytracer Object Collection
pub static OBJ_COLLECTION: Lazy<Mutex<Vec<RaytracerObjects>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

/// Raytracer Object Constants (starts with the OBJC_ prefix)
pub const OBJC_MAX_OBJ_COUNT: i32 = 100;
pub const OBJC_MAX_RAY_COUNT: i32 = 25;

/// Raytracer Default Object Parameters (starts with OBJD_ prefix)
pub const OBJD_CIRCLE_RADIUS: f32 = 50.0;
pub const OBJD_CIRCLE_FILL: Color = CORNFLOWER_BLUE;
pub const OBJD_RAY_WIDTH: f32 = 10.0;
pub const OBJD_RAY_COLOR: Color = CORNFLOWER_BLUE;

/// Raytracer Keybinds (starts with KEYB_ prefix)
pub const KEYB_SIMPLE_CIRCLE: KeyCode = KeyCode::S;
pub const KEYB_EMITTER_POINT: KeyCode = KeyCode::P;
