use macroquad::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Tile {
    walls: HashSet<Wall>,
    position: Vec2,
}

impl Tile {
    pub fn new(x: f32, y: f32) -> Self {
        let mut walls: HashSet<Wall> = HashSet::new();
        walls.insert(Wall::Left);
        walls.insert(Wall::Top);
        walls.insert(Wall::Right);
        walls.insert(Wall::Bottom);
        Self {
            walls,
            position: (x, y).into(),
        }
    }

    pub fn remove_wall(&mut self, wall: &Wall) -> bool {
        self.walls.remove(wall)
    }

    pub fn draw(&self, w: f32, h: f32, color: Color, material: &Material) {
        for wall in &self.walls {
            material.set_uniform("border_side", wall);
            material.set_uniform("tile_color", color.to_vec());
            material.set_uniform("border_color", BLUE.to_vec());
        }
        gl_use_material(&material);
        draw_rectangle(self.position.x, self.position.y, w, h, color);
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Wall {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
}

#[macroquad::main("Pixel Art Tiles")]
async fn main() {
    // Load shader files
    let vertex_shader = include_str!("shaders/vertex.glsl");
    let fragment_shader = include_str!("shaders/border.glsl");

    // Create a single material with our unified shader
    let material = load_material(
        ShaderSource::Glsl {
            vertex: vertex_shader,
            fragment: fragment_shader,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("border_side", UniformType::Int1),
                UniformDesc::new("tile_color", UniformType::Float4),
                UniformDesc::new("border_color", UniformType::Float4),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        clear_background(BLACK);

        //// Calculate positions and size for our four tiles
        //let tile_size = 100.0; // Size in screen pixels
        //let spacing = 20.0;
        //let start_x = screen_width() / 2.0 - tile_size - spacing / 2.0;
        //let start_y = screen_height() / 2.0 - tile_size - spacing / 2.0;
        //
        //// Draw bottom-left tile (left border)
        //material.set_uniform("border_side", 1);
        //gl_use_material(&material);
        //draw_rectangle(
        //    start_x,
        //    start_y + tile_size + spacing,
        //    tile_size,
        //    tile_size,
        //    WHITE,
        //);
        //
        //// Draw top-left tile (top border)
        //material.set_uniform("border_side", 1);
        //gl_use_material(&material);
        //draw_rectangle(start_x, start_y, tile_size, tile_size, WHITE);
        //
        //// Draw top-right tile (right border)
        //material.set_uniform("border_side", 2);
        //gl_use_material(&material);
        //draw_rectangle(
        //    start_x + tile_size + spacing,
        //    start_y,
        //    tile_size,
        //    tile_size,
        //    WHITE,
        //);
        //
        //// Draw bottom-right tile (bottom border)
        //material.set_uniform("border_side", 3);
        //gl_use_material(&material);
        //draw_rectangle(
        //    start_x + tile_size + spacing,
        //    start_y + tile_size + spacing,
        //    tile_size,
        //    tile_size,
        //    WHITE,
        //);

        let test = Tile::new(50.0, 50.0);
        test.draw(250.0, 250.0, RED, &material);

        // Reset to default material
        gl_use_default_material();

        // Draw labels for each tile
        //let font_size = 16.0;
        //draw_text("Top Border", start_x, start_y - 5.0, font_size, WHITE);
        //draw_text(
        //    "Right Border",
        //    start_x + tile_size + spacing,
        //    start_y - 5.0,
        //    font_size,
        //    WHITE,
        //);
        //draw_text(
        //    "Bottom Border",
        //    start_x + tile_size + spacing,
        //    start_y + tile_size * 2.0 + spacing + 15.0,
        //    font_size,
        //    WHITE,
        //);
        //draw_text(
        //    "Left Border",
        //    start_x,
        //    start_y + tile_size * 2.0 + spacing + 15.0,
        //    font_size,
        //    WHITE,
        //);

        next_frame().await
    }
}
