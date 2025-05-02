use macroquad::prelude::*;

use std::collections::HashSet;

use std::hash::{Hash, Hasher};

const PATH_COLOR: Color = BROWN;
pub const WALL_COLOR: Color = DARKGRAY;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Wall {
    Left = 1,
    Top = 2,
    Right = 4,
    Bottom = 8,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub col: usize,
    pub row: usize,
    pub walls: HashSet<Wall>,
    pub screen_position: Vec2,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub exit: bool,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row
    }
}

impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.row + 1 * self.col + 1).hash(state);
    }
}

impl Tile {
    pub fn new(
        col: usize,
        row: usize,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    ) -> Self {
        let mut walls: HashSet<Wall> = HashSet::new();
        walls.insert(Wall::Left);
        walls.insert(Wall::Top);
        walls.insert(Wall::Right);
        walls.insert(Wall::Bottom);
        Self {
            col,
            row,
            walls,
            screen_position: (x, y).into(),
            width,
            height,
            color,
            exit: false,
        }
    }

    pub fn remove_wall(&mut self, wall: &Wall) -> bool {
        self.color = PATH_COLOR;
        self.walls.remove(wall)
    }

    pub fn draw(&self, material: &Material) {
        let mut walls_sum: i32 = 0;
        for wall in &self.walls {
            walls_sum = walls_sum + (*wall as i32);
        }
        let pixels: f32 = 8.0;
        material.set_uniform("pixels", pixels);
        material.set_uniform("border_side", walls_sum);
        material.set_uniform("tile_color", self.color.to_vec());
        material.set_uniform("border_color", WALL_COLOR.to_vec());
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
