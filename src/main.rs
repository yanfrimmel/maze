use macroquad::prelude::*;

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
            uniforms: vec![UniformDesc::new("border_side", UniformType::Int1)],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        clear_background(BLACK);

        // Calculate positions and size for our four tiles
        let tile_size = 100.0; // Size in screen pixels
        let spacing = 20.0;
        let start_x = screen_width() / 2.0 - tile_size - spacing / 2.0;
        let start_y = screen_height() / 2.0 - tile_size - spacing / 2.0;

        // Draw top-left tile (top border)
        material.set_uniform("border_side", 0);
        gl_use_material(&material);
        draw_rectangle(start_x, start_y, tile_size, tile_size, WHITE);

        // Draw top-right tile (right border)
        material.set_uniform("border_side", 1);
        gl_use_material(&material);
        draw_rectangle(
            start_x + tile_size + spacing,
            start_y,
            tile_size,
            tile_size,
            WHITE,
        );

        // Draw bottom-right tile (bottom border)
        material.set_uniform("border_side", 2);
        gl_use_material(&material);
        draw_rectangle(
            start_x + tile_size + spacing,
            start_y + tile_size + spacing,
            tile_size,
            tile_size,
            WHITE,
        );

        // Draw bottom-left tile (left border)
        material.set_uniform("border_side", 3);
        gl_use_material(&material);
        draw_rectangle(
            start_x,
            start_y + tile_size + spacing,
            tile_size,
            tile_size,
            WHITE,
        );

        // Reset to default material
        gl_use_default_material();

        // Draw labels for each tile
        let font_size = 16.0;
        draw_text("Top Border", start_x, start_y - 5.0, font_size, WHITE);
        draw_text(
            "Right Border",
            start_x + tile_size + spacing,
            start_y - 5.0,
            font_size,
            WHITE,
        );
        draw_text(
            "Bottom Border",
            start_x + tile_size + spacing,
            start_y + tile_size * 2.0 + spacing + 15.0,
            font_size,
            WHITE,
        );
        draw_text(
            "Left Border",
            start_x,
            start_y + tile_size * 2.0 + spacing + 15.0,
            font_size,
            WHITE,
        );

        next_frame().await
    }
}

