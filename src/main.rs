use macroquad::prelude::*;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

const NUMBER_OF_TILES_IN_BIGGER_AXIS: u16 = 30;
const PATH_COLOR: Color = BROWN;
const WALL_COLOR: Color = DARKGRAY;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

// Player struct
#[derive(Debug)]
pub struct Player {
    // Grid position
    tile_pos: (usize, usize), // (col, row)
    screen_pos: Vec2,
    // Movement speed (pixels per second)
    speed: f32,
    radius: f32,
    color: Color,
    current_direction: Direction,
    tile_size: f32,
}

impl Player {
    pub fn new(col: usize, row: usize, tile_size: f32, first_x: f32, first_y: f32) -> Self {
        // Calculate initial screen position (center of the starting tile)
        let screen_x = first_x + (col as f32 * tile_size) + (tile_size / 2.0);
        let screen_y = first_y + (row as f32 * tile_size) + (tile_size / 2.0);

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

    pub fn update(&mut self, dt: f32, tiles: &Vec2d<Tile>, first_x: f32, first_y: f32) {
        if self.current_direction == Direction::None {
            // Not moving, make sure we're centered on the tile
            self.center_on_tile(first_x, first_y);
            return;
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

    pub fn get_tile_position(&self) -> (usize, usize) {
        self.tile_pos
    }
}

// Direction button for on-screen controls
#[derive(Debug)]
pub struct DirectionButton {
    rect: Rect,
    direction: Direction,
    color: Color,
    hover_color: Color,
    pressed_color: Color,
    is_pressed: bool,
}

impl DirectionButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32, direction: Direction) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            direction,
            color: Color::new(0.5, 0.5, 0.5, 0.7), // Semi-transparent gray
            hover_color: Color::new(0.6, 0.6, 0.6, 0.8),
            pressed_color: Color::new(0.4, 0.4, 0.4, 0.9),
            is_pressed: false,
        }
    }

    pub fn update(&mut self) -> Option<Direction> {
        let mouse_pos = mouse_position();
        let mouse_point = Vec2::new(mouse_pos.0, mouse_pos.1);
        let was_pressed = self.is_pressed;

        if self.rect.contains(mouse_point) {
            if is_mouse_button_down(MouseButton::Left) {
                self.is_pressed = true;
                return Some(self.direction);
            } else {
                self.is_pressed = false;
                if was_pressed {
                    return Some(Direction::None); // Button released
                }
            }
        } else if self.is_pressed && !is_mouse_button_down(MouseButton::Left) {
            self.is_pressed = false;
            return Some(Direction::None); // Button released (mouse moved off while pressed)
        }

        None
    }

    pub fn draw(&self) {
        let color = if self.is_pressed {
            self.pressed_color
        } else if self
            .rect
            .contains(Vec2::new(mouse_position().0, mouse_position().1))
        {
            self.hover_color
        } else {
            self.color
        };

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);

        // Draw direction arrow
        let center_x = self.rect.x + self.rect.w / 2.0;
        let center_y = self.rect.y + self.rect.h / 2.0;
        let arrow_size = self.rect.w.min(self.rect.h) * 0.5;

        match self.direction {
            Direction::Up => {
                draw_triangle(
                    Vec2::new(center_x, center_y - arrow_size / 2.0),
                    Vec2::new(center_x - arrow_size / 2.0, center_y + arrow_size / 2.0),
                    Vec2::new(center_x + arrow_size / 2.0, center_y + arrow_size / 2.0),
                    BLACK,
                );
            }
            Direction::Right => {
                draw_triangle(
                    Vec2::new(center_x + arrow_size / 2.0, center_y),
                    Vec2::new(center_x - arrow_size / 2.0, center_y - arrow_size / 2.0),
                    Vec2::new(center_x - arrow_size / 2.0, center_y + arrow_size / 2.0),
                    BLACK,
                );
            }
            Direction::Down => {
                draw_triangle(
                    Vec2::new(center_x, center_y + arrow_size / 2.0),
                    Vec2::new(center_x - arrow_size / 2.0, center_y - arrow_size / 2.0),
                    Vec2::new(center_x + arrow_size / 2.0, center_y - arrow_size / 2.0),
                    BLACK,
                );
            }
            Direction::Left => {
                draw_triangle(
                    Vec2::new(center_x - arrow_size / 2.0, center_y),
                    Vec2::new(center_x + arrow_size / 2.0, center_y - arrow_size / 2.0),
                    Vec2::new(center_x + arrow_size / 2.0, center_y + arrow_size / 2.0),
                    BLACK,
                );
            }
            Direction::None => {}
        }
    }
}

// Control pad with all four direction buttons
pub struct ControlPad {
    buttons: [DirectionButton; 4],
}

impl ControlPad {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        let button_size = size / 3.0;

        // Create four directional buttons in a cross pattern
        let up = DirectionButton::new(x + button_size, y, button_size, button_size, Direction::Up);

        let right = DirectionButton::new(
            x + button_size * 2.0,
            y + button_size,
            button_size,
            button_size,
            Direction::Right,
        );

        let down = DirectionButton::new(
            x + button_size,
            y + button_size * 2.0,
            button_size,
            button_size,
            Direction::Down,
        );

        let left = DirectionButton::new(
            x,
            y + button_size,
            button_size,
            button_size,
            Direction::Left,
        );

        Self {
            buttons: [up, right, down, left],
        }
    }

    pub fn update(&mut self, player: &mut Player) {
        for button in &mut self.buttons {
            if let Some(direction) = button.update() {
                player.set_direction(direction);
            }
        }

        // Also check keyboard input
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            player.set_direction(Direction::Up);
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player.set_direction(Direction::Right);
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            player.set_direction(Direction::Down);
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player.set_direction(Direction::Left);
        }
    }

    pub fn draw(&self) {
        for button in &self.buttons {
            button.draw();
        }
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
    col: usize,
    row: usize,
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
            col,
            row,
            walls,
            screen_position: (x, y).into(),
            width,
            height,
            color,
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

fn generate_tiles() -> Vec2d<Tile> {
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
    let max_steps = NUMBER_OF_TILES_IN_BIGGER_AXIS / 10;

    let interval = 0.1 / (NUMBER_OF_TILES_IN_BIGGER_AXIS as f64);
    let mut run_time: f64 = interval;
    let mut generation_done = false;

    let first_tile = tiles.vec.get(0).unwrap();
    let first_tile_pos = first_tile.screen_position.clone();
    let mut player: Player = Player::new(
        0,
        0,
        first_tile.width,
        first_tile.screen_position.x,
        first_tile.screen_position.y,
    );

    // Create control pad
    let control_size = screen_height() * 0.25;
    let controls_x = screen_width() - control_size - 20.0;
    let controls_y = screen_height() - control_size - 20.0;
    let mut control_pad = ControlPad::new(controls_x, controls_y, control_size);

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        if !generation_done {
            let seconds_passed = get_time();

            if seconds_passed >= run_time && visited.len() != tiles_len {
                start_position = iterative_backtracking(
                    &mut tiles,
                    &mut visited,
                    &mut stack,
                    start_position,
                    max_steps as _,
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

        // Reset to default material
        gl_use_default_material();

        if generation_done {
            // Handle keyboard input as an alternative to on-screen buttons
            control_pad.update(&mut player);
            player.update(dt, &tiles, first_tile_pos.x, first_tile_pos.y);
            player.draw();
            control_pad.draw();
        }

        draw_fps();
        next_frame().await
    }
}
