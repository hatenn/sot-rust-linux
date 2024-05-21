
type Colour = [f32; 4];
pub const RED: Colour = [1.0, 0.0, 0.0, 1.0];
pub const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
pub const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];

pub const WHITE: Colour = [1.0; 4];
pub const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
pub const BROWN : Colour = [0.70, 0.42, 0.31, 0.8];
pub const BACKGROUND: Colour = [0.0, 0.0, 0., 0.0]; //Transparent
pub const CORAL: Colour = [0.55, 0.92, 0.08, 1.0];
pub const LAVENDER: Colour = [0.28, 0.07, 0.51, 0.71]; // Semi-transparent
pub const MINT: Colour = [0.16, 0.99, 0.44, 1.0];
pub const SUNSET: Colour = [0.62, 0.03, 0.87, 1.0];
pub const TEAL: Colour = [0.14, 0.9, 0.63, 1.0];
pub const LEMON: Colour = [0.98, 0.86, 0.07, 1.0];
pub const GOLDENROD: Colour = [0.9, 0.8, 0.1, 1.0];
pub const SUNSHINE: Colour = [0.99, 0.91, 0.17, 1.0];


#[derive(Clone)]
pub enum ObjectType{
    Circle,
    Box,
    Text
}

#[derive(Clone)]
pub struct Actor {
    pub x: f64,
    pub y: f64,
    pub width : f64,
    pub height : f64,
    pub object_type : ObjectType,
    pub name: String,
    pub font_size : u32,
    pub colour: Colour,
}

impl Actor {
    pub fn new(x: f64, y: f64, width : f64, height : f64, name: String, font_size : u32, object_type : ObjectType, colour: Colour) -> Self {
        Actor {
            x,
            y,
            width,
            height,
            object_type,
            name,
            font_size,
            colour,
        }
    }
}