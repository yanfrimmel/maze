use crate::player::{Direction, Player};

use macroquad::prelude::*;

#[derive(Debug)]
pub struct DirectionButton {
    rect: Rect,
    direction: Direction,
    color: Color,
    hover_color: Color,
    pressed_color: Color,
    triangle_color: Color,
    is_pressed: bool,
}

impl DirectionButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32, direction: Direction) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            direction,
            color: Color::new(0.5, 0.5, 0.5, 0.3), // Semi-transparent gray
            hover_color: Color::new(0.6, 0.6, 0.6, 0.4),
            pressed_color: Color::new(0.4, 0.4, 0.4, 0.5),
            triangle_color: Color::new(0.0, 0.0, 0.0, 0.2),
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
                    self.triangle_color,
                );
            }
            Direction::Right => {
                draw_triangle(
                    Vec2::new(center_x + arrow_size / 2.0, center_y),
                    Vec2::new(center_x - arrow_size / 2.0, center_y - arrow_size / 2.0),
                    Vec2::new(center_x - arrow_size / 2.0, center_y + arrow_size / 2.0),
                    self.triangle_color,
                );
            }
            Direction::Down => {
                draw_triangle(
                    Vec2::new(center_x, center_y + arrow_size / 2.0),
                    Vec2::new(center_x - arrow_size / 2.0, center_y - arrow_size / 2.0),
                    Vec2::new(center_x + arrow_size / 2.0, center_y - arrow_size / 2.0),
                    self.triangle_color,
                );
            }
            Direction::Left => {
                draw_triangle(
                    Vec2::new(center_x - arrow_size / 2.0, center_y),
                    Vec2::new(center_x + arrow_size / 2.0, center_y - arrow_size / 2.0),
                    Vec2::new(center_x + arrow_size / 2.0, center_y + arrow_size / 2.0),
                    self.triangle_color,
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
            match button.update() {
                Some(direction) => {
                    if direction != Direction::None {
                        player.set_direction(direction);
                    }
                }
                None => {}
            }
        }

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
