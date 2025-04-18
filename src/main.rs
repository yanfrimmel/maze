use macroquad::prelude::*;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Wall {
    Left = 1,
    Top = 2,
    Right = 4,
    Bottom = 8,
}

#[derive(Debug)]
pub struct Tile {
    walls: HashSet<Wall>,
    screen_position: Vec2,
    width: f32,
    height: f32,
    color: Color,
}

impl Tile {
    pub fn new(x: f32, y: f32, w: f32, h: f32, c: Color) -> Self {
        let mut walls: HashSet<Wall> = HashSet::new();
        walls.insert(Wall::Left);
        walls.insert(Wall::Top);
        walls.insert(Wall::Right);
        walls.insert(Wall::Bottom);
        Self {
            walls,
            screen_position: (x, y).into(),
            width: w,
            height: h,
            color: c,
        }
    }

    pub fn remove_wall(&mut self, wall: &Wall) -> bool {
        self.walls.remove(wall)
    }

    pub fn draw(&self, material: &Material) {
        let mut walls_sum: i32 = 0;
        for wall in &self.walls {
            walls_sum = walls_sum + (*wall as i32);
        }
        material.set_uniform("border_side", walls_sum);
        material.set_uniform("tile_color", self.color.to_vec());
        material.set_uniform("border_color", DARKGRAY.to_vec());
        gl_use_material(&material);
        draw_rectangle(
            self.screen_position.x,
            self.screen_position.y,
            self.width,
            self.height,
            self.color,
        );
    }
}

#[macroquad::main("Pixel Art Tiles")]
async fn main() {
    // Load shader files
    let vertex_shader = include_str!("shaders/vertex.glsl");
    let fragment_shader = include_str!("shaders/border.glsl");

    // Create a single material with our unified shader
    let tile_material = load_material(
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

        let mut test = Tile::new(100.0, 100.0, 100.0, 100.0, BROWN);
        test.remove_wall(&Wall::Left);
        test.draw(&tile_material);

        let mut test2 = Tile::new(200.0, 100.0, 100.0, 100.0, BROWN);
        test2.remove_wall(&Wall::Top);
        test2.draw(&tile_material);

        let mut test3 = Tile::new(100.0, 200.0, 100.0, 100.0, BROWN);
        test3.remove_wall(&Wall::Bottom);
        test3.draw(&tile_material);

        let mut test4 = Tile::new(200.0, 200.0, 100.0, 100.0, BROWN);
        test4.remove_wall(&Wall::Right);
        test4.remove_wall(&Wall::Left);
        test4.remove_wall(&Wall::Bottom);
        test4.remove_wall(&Wall::Top);
        test4.draw(&tile_material);

        // Reset to default material
        gl_use_default_material();

        next_frame().await
    }
}
