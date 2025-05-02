use crate::tile::{Tile, Wall};
use crate::utils::Vec2d;

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

#[derive(Debug)]
pub struct Player {
    // Grid position
    pub tile_pos: (usize, usize), // (col, row)
    pub screen_pos: Vec2,
    // Movement speed (pixels per second)
    pub speed: f32,
    pub radius: f32,
    pub color: Color,
    pub current_direction: Direction,
    pub tile_size: f32,
}

impl Player {
    pub fn new(col: usize, row: usize, tile_size: f32, screen_x: f32, screen_y: f32) -> Self {
        Self {
            tile_pos: (col, row),
            screen_pos: Vec2::new(screen_x, screen_y),
            speed: tile_size * 4.0, // Move at 4 tiles per second
            radius: tile_size * 0.25,
            color: YELLOW,
            current_direction: Direction::None,
            tile_size,
        }
    }

    pub fn draw(&self) {
        draw_circle(
            self.screen_pos.x,
            self.screen_pos.y,
            self.radius,
            self.color,
        );
    }

    // returns if found exit
    pub fn update(&mut self, dt: f32, tiles: &Vec2d<Tile>, first_x: f32, first_y: f32) -> bool {
        if tiles.index(self.tile_pos.0, self.tile_pos.1).exit {
            return true;
        }

        if self.current_direction == Direction::None {
            // Not moving, make sure we're centered on the tile
            self.center_on_tile(first_x, first_y);
            return false;
        }

        // Calculate movement vector based on direction
        let move_vector = match self.current_direction {
            Direction::Up => Vec2::new(0.0, -1.0),
            Direction::Right => Vec2::new(1.0, 0.0),
            Direction::Down => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::None => Vec2::new(0.0, 0.0),
        };

        // Calculate new position
        let new_pos = self.screen_pos + move_vector * self.speed * dt;

        // Calculate grid position from screen position (accounting for offset)
        let grid_col = ((new_pos.x - first_x) / self.tile_size).floor() as usize;
        let grid_row = ((new_pos.y - first_y) / self.tile_size).floor() as usize;

        // Check for wall collisions
        let can_move = match self.current_direction {
            Direction::Up => !tiles
                .index(self.tile_pos.0, self.tile_pos.1)
                .walls
                .contains(&Wall::Top),
            Direction::Right => !tiles
                .index(self.tile_pos.0, self.tile_pos.1)
                .walls
                .contains(&Wall::Right),
            Direction::Down => !tiles
                .index(self.tile_pos.0, self.tile_pos.1)
                .walls
                .contains(&Wall::Bottom),
            Direction::Left => !tiles
                .index(self.tile_pos.0, self.tile_pos.1)
                .walls
                .contains(&Wall::Left),
            Direction::None => true,
        };

        if can_move {
            // Update screen position
            self.screen_pos = new_pos;

            // Update tile position if changed
            if grid_col != self.tile_pos.0 || grid_row != self.tile_pos.1 {
                // Make sure the new position is within the maze bounds
                if grid_col < tiles.cols && grid_row < tiles.rows {
                    self.tile_pos = (grid_col, grid_row);
                }
            }
        } else {
            // Can't move in this direction, stop and center on current tile
            self.current_direction = Direction::None;
            self.center_on_tile(first_x, first_y);
        }
        false
    }

    fn center_on_tile(&mut self, first_x: f32, first_y: f32) {
        // Calculate center position of current tile
        let center_x = first_x + (self.tile_pos.0 as f32 * self.tile_size) + (self.tile_size / 2.0);
        let center_y = first_y + (self.tile_pos.1 as f32 * self.tile_size) + (self.tile_size / 2.0);

        // Smoothly move toward center
        let dt = get_frame_time();
        self.screen_pos.x = self.screen_pos.x + (center_x - self.screen_pos.x) * 10.0 * dt;
        self.screen_pos.y = self.screen_pos.y + (center_y - self.screen_pos.y) * 10.0 * dt;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.current_direction = direction;
    }
}
