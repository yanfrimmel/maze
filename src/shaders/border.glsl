#version 100
precision mediump float;

varying vec2 uv;

// Parameter to control which side has the border:
// 0 = top, 1 = right, 2 = bottom, 3 = left
uniform int border_side;

const vec3 tile_color = vec3(0.2, 0.6, 0.3);  // Green for the tile
const vec3 border_color = vec3(0.8, 0.2, 0.2);  // Red for the border

void main() {
    // Tile is 16x16 pixels
    vec2 tile_size = vec2(16.0, 16.0);
    
    // Calculate pixel position within the tile (0-15 for each axis)
    vec2 tile_pixel = floor(uv * tile_size);
    
    // Default color is the tile color
    vec3 color = tile_color;
   
    // Apply border based on the border_side parameter
    if (border_side == 0 && tile_pixel.y == 0.0) {
        // Top border
        color = border_color;
    } else if (border_side == 1 && tile_pixel.x == 15.0) {
        // Right border
        color = border_color;
    } else if (border_side == 2 && tile_pixel.y == 15.0) {
        // Bottom border
        color = border_color;
    } else if (border_side == 3 && tile_pixel.x == 0.0) {
        // Left border
        color = border_color;
    }
    
    gl_FragColor = vec4(color, 1.0);
}
