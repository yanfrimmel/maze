#version 100
precision mediump float;

varying vec2 uv;

// Parameter to control which side has the border:
// 0 = left, 1 = top, 2 = top, 3 = bottom
uniform int border_side;
uniform vec4 tile_color;
uniform vec4 border_color;

void main() {
    // Tile is 16x16 pixels
    vec2 tile_size = vec2(16.0, 16.0);
    
    // Calculate pixel position within the tile (0-15 for each axis)
    vec2 tile_pixel = floor(uv * tile_size);
    
    // Default color is the tile color
    vec4 color = tile_color;
   
    // Apply border based on the border_side parameter
    if (border_side == 0 && tile_pixel.x == 0.0) {
        // Left border
        color = border_color;
    } else if (border_side == 1 && tile_pixel.y == 0.0) {
        // Top border
        color = border_color;
    } else if (border_side == 2 && tile_pixel.x == 15.0) {
        // Right border
        color = border_color;
    } else if (border_side == 3 && tile_pixel.y == 15.0) {
        // Bottom border
        color = border_color;
    }

    gl_FragColor = color;
}
