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

    pub fn index(&self, row: usize, col: usize) -> &T {
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
        let pixels: f32 = 8.0;
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
    let tile_size: u16 = 100;
    let s_w = screen_width();
    let s_h = screen_height();

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

pub fn iterative_backtracking(tiles: &mut Vec2d<Tile>) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();

    // Choose random starting position
    rand::srand(10);
    let mut curr_row = rand::gen_range(0, tiles.rows);
    let mut curr_col = rand::gen_range(0, tiles.cols);
    stack.push((curr_col, curr_row));
    visited.insert((curr_col, curr_row));
    let len = tiles.vec.len();

    while visited.len() != len {
        let mut neighbors =
            get_unvisited_neighbors(curr_col, curr_row, tiles.cols, tiles.rows, &visited);
        println!("curr (col, row): {:?}", (curr_col, curr_row));
        println!("neighbors: {:?}", neighbors);
        if neighbors.len() != 0 {
            neighbors.shuffle();
            let (nc, nr) = neighbors[0];
            remove_walls_between_positions(tiles, (curr_col, curr_row), (nc, nr));

            println!(
                "carve: cur: {:?}, other: {:?}",
                (curr_col, curr_row),
                (nc, nr)
            );

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

fn get_all_neighbors(
    row: usize,
    col: usize,
    max_rows: usize,
    max_cols: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::with_capacity(4);

    if row > 0 {
        neighbors.push((row - 1, col));
    } // North
    if row < max_rows - 1 {
        neighbors.push((row + 1, col));
    } // South
    if col > 0 {
        neighbors.push((row, col - 1));
    } // West
    if col < max_cols - 1 {
        neighbors.push((row, col + 1));
    } // East

    neighbors
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

    println!("cols: {}", tiles.cols);
    println!("rows: {}", tiles.rows);
    println!("tiles: {}", tiles.vec.len());

    iterative_backtracking(&mut tiles);

    loop {
        clear_background(BLACK);

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

        next_frame().await
    }
}
