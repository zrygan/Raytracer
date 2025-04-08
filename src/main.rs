use macroquad::prelude::*;

mod objects;
use objects::{circle::ObjectCircle, object_traits::Drawable};

#[macroquad::main("Raytracer [release: rust-rewrite]")]
async fn main(){
    loop {
        clear_background(BLACK);
        // Uses macroquad draw circle
        draw_circle(screen_height()/20.0, screen_height()/20.0, 10.0, RED);
        
        // using raytracer draw circle
        let new_circle = ObjectCircle::new(10.0, 10.0, Color { r: 0.5, g: 0.5, b: 0.5, a: 0.5 }, 10.0);
        new_circle.draw_object();
        next_frame().await;
    }
}