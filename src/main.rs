mod controls;
mod maze;
mod player;
mod tile;
mod utils;

use controls::ControlPad;
use maze::*;
use player::Player;

use macroquad::prelude::*;
use std::collections::HashSet;

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
        first_tile.screen_position.x + (0 as f32 * first_tile.width) + (first_tile.width / 2.0),
        first_tile.screen_position.y + (0 as f32 * first_tile.width) + (first_tile.width / 2.0),
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
                choose_exit_tile(&mut tiles);
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
            player.draw();
            control_pad.draw();
            if player.update(dt, &tiles, first_tile_pos.x, first_tile_pos.y) {
                generation_done = false;
                tiles = generate_tiles();
                visited.clear();
                stack.clear();
                let start_row = rand::gen_range(0, tiles.rows);
                let start_col = rand::gen_range(0, tiles.cols);
                start_position = (start_col, start_row);
                player = Player::new(
                    player.tile_pos.0,
                    player.tile_pos.1,
                    player.tile_size,
                    player.screen_pos.x,
                    player.screen_pos.y,
                )
            }
        }

        draw_fps();
        next_frame().await
    }
}
