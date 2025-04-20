use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Vec2d<T> {
    pub fn new(vec: Vec<T>, rows: usize, cols: usize) -> Self {
        assert!(vec.len() == rows * cols);
        Self { vec, rows, cols }
    }

    pub fn row(&self, row: usize) -> &[T] {
        let i = self.cols * row;
        &self.vec[i..(i + self.cols)]
    }

    pub fn index(&self, col: usize, row: usize) -> &T {
        let i = self.cols * row;
        &self.vec[i + col]
    }

    pub fn index_mut(&mut self, col: usize, row: usize) -> &mut T {
        let i = self.cols * row;
        &mut self.vec[i + col]
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.rows {
            if i != 0 {
                str.push_str(", ");
            }
            str.push_str(&format!("{:?}", &self.row(i)));
        }
        write!(f, "[{}]", str)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Wall {
    Left = 1,
    Top = 2,
    Right = 4,
    Bottom = 8,
}

#[derive(Debug, Clone)]
pub struct Tile {
    row: usize,
    col: usize,
    walls: HashSet<Wall>,
    screen_position: Vec2,
    width: f32,
    height: f32,
    color: Color,
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
            row,
            col,
            walls,
            screen_position: (x, y).into(),
            width,
            height,
            color,
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
        let pixels: f32 = 16.0;
        material.set_uniform("pixels", pixels);
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

fn generate_tiles() -> Vec2d<Tile> {
    let s_w = screen_width();
    let s_h = screen_height();
    let number_of_tiles_in_bigger_axis = 20;
    let tile_size;

    if s_w > s_h {
        tile_size = (s_w / number_of_tiles_in_bigger_axis as f32) as u16;
    } else {
        tile_size = (s_h / number_of_tiles_in_bigger_axis as f32) as u16;
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
                BROWN,
            ));
        }
    }

    Vec2d::new(tiles, tiles_h as usize, tiles_w as usize)
}

/// Performs maze generation using the iterative backtracking algorithm with a step limit
///
/// Parameters:
/// - tiles: The 2D grid of tiles to carve the maze into
/// - visited: HashSet tracking which cells have been visited
/// - stack: Stack used for backtracking
/// - start_position: The starting position (column, row) for maze generation
/// - max_steps: Maximum number of steps to take before stopping (0 for unlimited)
///
/// Returns:
/// - The final position (column, row) where generation ended
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

        let mut neighbors =
            get_unvisited_neighbors(curr_col, curr_row, tiles.cols, tiles.rows, &visited);
        // println!("curr (col, row): {:?}", (curr_col, curr_row));
        // println!("neighbors: {:?}", neighbors);

        if neighbors.len() != 0 {
            neighbors.shuffle();
            let (nc, nr) = neighbors[0];
            remove_walls_between_positions(tiles, (curr_col, curr_row), (nc, nr));

            // println!(
            //     "carve: cur: {:?}, other: {:?}",
            //     (curr_col, curr_row),
            //     (nc, nr)
            // );
            stack.push((curr_col, curr_row));
            (curr_col, curr_row) = (nc, nr);
            visited.insert((nc, nr));
        } else if stack.len() != 0 {
            (curr_col, curr_row) = stack.pop().unwrap();
        } else {
            panic!("Infinite loop");
        }
    }

    (curr_col, curr_row)
}

pub fn iterative_backtracking_old(tiles: &mut Vec2d<Tile>) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();

    // Choose random starting position
    let mut curr_row = rand::gen_range(0, tiles.rows);
    let mut curr_col = rand::gen_range(0, tiles.cols);
    stack.push((curr_col, curr_row));
    visited.insert((curr_col, curr_row));
    let len = tiles.vec.len();

    while visited.len() != len {
        let mut neighbors =
            get_unvisited_neighbors(curr_col, curr_row, tiles.cols, tiles.rows, &visited);
        // println!("curr (col, row): {:?}", (curr_col, curr_row));
        // println!("neighbors: {:?}", neighbors);
        if neighbors.len() != 0 {
            neighbors.shuffle();
            let (nc, nr) = neighbors[0];
            remove_walls_between_positions(tiles, (curr_col, curr_row), (nc, nr));
            // println!(
            //     "carve: cur: {:?}, other: {:?}",
            //     (curr_col, curr_row),
            //     (nc, nr)
            // );
            stack.push((curr_col, curr_row));
            (curr_col, curr_row) = (nc, nr);
            visited.insert((nc, nr));
            // println!("visited: (curr_row, curr_col): {:?}", (curr_row, curr_col));
        } else if stack.len() != 0 {
            (curr_col, curr_row) = stack.pop().unwrap();
            // println!(
            //     "stack pop: (curr_row, curr_col): {:?}",
            //     (curr_row, curr_col)
            // );
        } else {
            panic!("infinite loop");
            // println!("(curr_row, curr_col): {:?}", (curr_row, curr_col));
        }
    }
    // println!("Result: {:?}", visited);
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

fn remove_walls_between_positions(
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

#[macroquad::main("Maze")]
async fn main() {
    let time = macroquad::miniquad::date::now();

    println!("Rand seed: {}", time);
    rand::srand(time as _);

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
                UniformDesc::new("pixels", UniformType::Float1),
                UniformDesc::new("border_side", UniformType::Int1),
                UniformDesc::new("tile_color", UniformType::Float4),
                UniformDesc::new("border_color", UniformType::Float4),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut tiles = generate_tiles();
    let tiles_len = tiles.vec.len();

    println!("cols: {}", tiles.cols);
    println!("rows: {}", tiles.rows);
    println!("tiles: {}", tiles.vec.len());

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();

    let start_row = rand::gen_range(0, tiles.rows);
    let start_col = rand::gen_range(0, tiles.cols);
    let mut start_position = (start_col, start_row);
    let max_steps = 1;

    let interval = 0.01;
    let mut run_time: f64 = interval;
    let mut generation_done = false;

    loop {
        clear_background(BLACK);

        if !generation_done {
            let seconds_passed = get_time();

            if seconds_passed >= run_time && visited.len() != tiles_len {
                start_position = iterative_backtracking(
                    &mut tiles,
                    &mut visited,
                    &mut stack,
                    start_position,
                    max_steps,
                );
                run_time = seconds_passed + interval;
            } else if visited.len() == tiles_len {
                let precentage = rand::gen_range(0.01, 0.05);
                remove_random_walls(&mut tiles, precentage);
                generation_done = true;
                println!("Maze generation done!")
            }
        }

        for tile in &tiles.vec {
            tile.draw(&tile_material);
        }

        // let mut test = Tile::new(100.0, 100.0, 100.0, 100.0, BROWN);
        // test.remove_wall(&Wall::Left);
        // test.draw(&tile_material);
        //
        // let mut test2 = Tile::new(200.0, 100.0, 100.0, 100.0, BROWN);
        // test2.remove_wall(&Wall::Top);
        // test2.draw(&tile_material);
        //
        // let mut test3 = Tile::new(100.0, 200.0, 100.0, 100.0, BROWN);
        // test3.remove_wall(&Wall::Bottom);
        // test3.draw(&tile_material);
        //
        // let mut test4 = Tile::new(200.0, 200.0, 100.0, 100.0, BROWN);
        // test4.remove_wall(&Wall::Right);
        // test4.remove_wall(&Wall::Left);
        // test4.remove_wall(&Wall::Bottom);
        // test4.remove_wall(&Wall::Top);
        // test4.draw(&tile_material);

        // Reset to default material
        gl_use_default_material();
        draw_fps();
        next_frame().await
    }
}
