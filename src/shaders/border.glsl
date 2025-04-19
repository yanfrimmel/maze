#version 100
precision mediump float;

varying vec2 uv;

// Parameter to control which side has the border:
// 1 = left, 2 = top, 4 = top, 8 = bottom
uniform float pixels;
uniform int border_side;
uniform vec4 tile_color;
uniform vec4 border_color;

void main() {
    vec2 tile_size = vec2(pixels, pixels);
    
    // Calculate pixel position within the tile (0-15 for each axis)
    vec2 tile_pixel = floor(uv * tile_size);
    
    // Default color is the tile color
    vec4 color = tile_color;
   
    // Apply border based on the border_side parameter
    int temp = border_side;
    if (mod(float(temp), 2.0) == 1.0 && tile_pixel.x == 0.0) {
        // Left border
        color = border_color;
    }
    temp = (temp / 2);
    if (mod(float(temp), 2.0) == 1.0 && tile_pixel.y == 0.0) {
        // Top border
        color = border_color;
    }
    temp = (temp / 2);
    if (mod(float(temp), 2.0) == 1.0 && tile_pixel.x == (pixels - 1.0)) {
        // Right border
        color = border_color;
    }
    temp = (temp / 2);
    if (mod(float(temp), 2.0) == 1.0 && tile_pixel.y == (pixels -1.0)) {
        // Bottom border
        color = border_color;
    }

    gl_FragColor = color;
}
