use crate::tile::{Tile, WALL_COLOR, Wall};
use crate::utils::Vec2d;

use macroquad::prelude::*;
use std::collections::HashSet;

pub const NUMBER_OF_TILES_IN_BIGGER_AXIS: u16 = 30;

pub fn generate_tiles() -> Vec2d<Tile> {
    let s_w = screen_width();
    let s_h = screen_height();
    let tile_size;

    if s_w > s_h {
        tile_size = (s_w / NUMBER_OF_TILES_IN_BIGGER_AXIS as f32) as u16;
    } else {
        tile_size = (s_h / NUMBER_OF_TILES_IN_BIGGER_AXIS as f32) as u16;
    }

    let tiles_w: u16 = s_w as u16 / tile_size;
    let tiles_h: u16 = s_h as u16 / tile_size;

    let reminder_w: u16 = s_w as u16 % tile_size;
    let reminder_h: u16 = s_h as u16 % tile_size;

    let first_x = reminder_w / 2;
    let first_y = reminder_h / 2;
    let mut tiles: Vec<Tile> = Vec::new();

    for y in 0..tiles_h {
        for x in 0..tiles_w {
            tiles.push(Tile::new(
                x as usize,
                y as usize,
                (x * tile_size + first_x) as f32,
                (y * tile_size + first_y) as f32,
                tile_size as f32,
                tile_size as f32,
                WALL_COLOR,
            ));
        }
    }

    Vec2d::new(tiles, tiles_h as usize, tiles_w as usize)
}

pub fn iterative_backtracking(
    tiles: &mut Vec2d<Tile>,
    visited: &mut HashSet<(usize, usize)>,
    stack: &mut Vec<(usize, usize)>,
    start_position: (usize, usize),
    max_steps: usize,
) -> (usize, usize) {
    let (mut curr_col, mut curr_row) = start_position;
    let mut steps_taken = 0;

    // Initialize if the stack is empty (first call)
    if stack.is_empty() {
        stack.push((curr_col, curr_row));
        visited.insert((curr_col, curr_row));
    }

    let len = tiles.vec.len();
    let unlimited = max_steps == 0;

    while visited.len() != len && (unlimited || steps_taken < max_steps) {
        steps_taken += 1;

        let neighbors =
            get_unvisited_neighbors(curr_col, curr_row, tiles.cols, tiles.rows, &visited);
        // println!("curr (col, row): {:?}", (curr_col, curr_row));
        // println!("neighbors: {:?}", neighbors);

        if !neighbors.is_empty() {
            let random_index = rand::gen_range(0, neighbors.len());
            let (nc, nr) = neighbors[random_index];
            remove_walls_between_positions(tiles, (curr_col, curr_row), (nc, nr));

            // println!(
            //     "carve: cur: {:?}, other: {:?}",
            //     (curr_col, curr_row),
            //     (nc, nr)
            // );
            stack.push((curr_col, curr_row));
            (curr_col, curr_row) = (nc, nr);
            visited.insert((nc, nr));
        } else if !stack.is_empty() {
            (curr_col, curr_row) = stack.pop().unwrap();
        } else {
            panic!("Infinite loop");
        }
    }

    (curr_col, curr_row)
}

pub fn remove_random_walls(tiles: &mut Vec2d<Tile>, percentage: f32) {
    // Ensure percentage is within valid range (0.0 to 1.0)
    let percentage = percentage.clamp(0.0, 1.0);

    // Get total number of potential internal walls to remove
    // Each tile has 4 walls but walls are shared, so count total connections
    let total_internal_walls = (tiles.rows - 1) * tiles.cols + (tiles.cols - 1) * tiles.rows;
    // Calculate how many walls to remove
    let walls_to_remove = (total_internal_walls as f32 * percentage) as usize;

    println!(
        "total_internal_walls: {}, walls_to_remove: {}",
        total_internal_walls, walls_to_remove
    );

    // Track which walls we've already removed
    let mut removed_connections: HashSet<((usize, usize), (usize, usize))> = HashSet::new();

    // Try to remove the specified number of walls
    let mut count = 0;
    let mut attempts = 0;
    let max_attempts = total_internal_walls * 5; // Limit attempts to avoid infinite loop

    while count < walls_to_remove && attempts < max_attempts {
        attempts += 1;

        // Pick a random tile
        let col = rand::gen_range(0, tiles.cols);
        let row = rand::gen_range(0, tiles.rows);

        // Pick a random direction (0=right, 1=bottom, 2=left, 3=top)
        let direction = rand::gen_range(0, 4);

        // Find neighboring tile in chosen direction
        let neighbor = match direction {
            0 => {
                if col < tiles.cols - 1 {
                    Some((col + 1, row))
                } else {
                    None
                }
            }
            1 => {
                if row < tiles.rows - 1 {
                    Some((col, row + 1))
                } else {
                    None
                }
            }
            2 => {
                if col > 0 {
                    Some((col - 1, row))
                } else {
                    None
                }
            }
            3 => {
                if row > 0 {
                    Some((col, row - 1))
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some((n_col, n_row)) = neighbor {
            // Order tile coordinates for consistent HashSet lookup
            let connection = if (col, row) < (n_col, n_row) {
                ((col, row), (n_col, n_row))
            } else {
                ((n_col, n_row), (col, row))
            };

            // Skip if we've already removed this wall
            if removed_connections.contains(&connection) {
                continue;
            }

            // Check if there's a wall between these tiles
            let has_wall = match direction {
                0 => tiles.index(col, row).walls.contains(&Wall::Right),
                1 => tiles.index(col, row).walls.contains(&Wall::Bottom),
                2 => tiles.index(col, row).walls.contains(&Wall::Left),
                3 => tiles.index(col, row).walls.contains(&Wall::Top),
                _ => false,
            };

            if has_wall {
                // Remove the wall between tiles
                remove_walls_between_positions(tiles, (col, row), (n_col, n_row));
                removed_connections.insert(connection);
                count += 1;
            }
        }
    }

    println!("Removed {} walls out of {} attempts", count, attempts);
}

pub fn remove_walls_between_positions(
    tiles: &mut Vec2d<Tile>,
    pos1: (usize, usize),
    pos2: (usize, usize),
) {
    let (col1, row1) = pos1;
    let (col2, row2) = pos2;

    if row1 == row2 {
        // Horizontal neighbors
        if col1 < col2 {
            tiles.index_mut(col1, row1).remove_wall(&Wall::Right);
            tiles.index_mut(col2, row2).remove_wall(&Wall::Left);
        } else {
            tiles.index_mut(col1, row1).remove_wall(&Wall::Left);
            tiles.index_mut(col2, row2).remove_wall(&Wall::Right);
        }
    } else {
        // Vertical neighbors
        if row1 < row2 {
            tiles.index_mut(col1, row1).remove_wall(&Wall::Bottom);
            tiles.index_mut(col2, row2).remove_wall(&Wall::Top);
        } else {
            tiles.index_mut(col1, row1).remove_wall(&Wall::Top);
            tiles.index_mut(col2, row2).remove_wall(&Wall::Bottom);
        }
    }
}

fn get_unvisited_neighbors(
    col: usize,
    row: usize,
    max_cols: usize,
    max_rows: usize,
    visited: &HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4);

    // Check all four directions
    if row > 0 && !visited.contains(&(col, row - 1)) {
        neighbors.push((col, row - 1));
    }
    if row < max_rows - 1 && !visited.contains(&(col, row + 1)) {
        neighbors.push((col, row + 1));
    }
    if col > 0 && !visited.contains(&(col - 1, row)) {
        neighbors.push((col - 1, row));
    }
    if col < max_cols - 1 && !visited.contains(&(col + 1, row)) {
        neighbors.push((col + 1, row));
    }

    neighbors
}

pub fn choose_exit_tile(tiles: &mut Vec2d<Tile>) {
    let col = rand::gen_range(0, tiles.cols);
    let row = rand::gen_range(0, tiles.rows);
    let tile = tiles.index_mut(col, row);
    tile.exit = true;
    tile.color = YELLOW;
}
