/// Drawable trait for the 

pub trait Drawable {
    fn draw_object(&self);
}

pub trait Movable {
    fn move_object(&mut self, pos_x: f32, pos_y: f32);
}