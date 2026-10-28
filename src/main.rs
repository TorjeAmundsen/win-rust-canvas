use std::{
    thread,
    time::{Duration, Instant},
};

mod color;
mod models;
mod point;
mod state;
mod win32;

use models::cube::{CUBE, CUBE_EDGES};
use models::penguin::{PENGUIN, PENGUIN_FACES};

use crate::color::Color;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;
const FPS: u64 = 120;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl Canvas {
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    pub fn draw_point(&mut self, x: usize, y: usize, color: u32, size: usize) {
        if size == 0 {
            return;
        }

        let radius = size - 1;

        let clamped_left = x.saturating_sub(radius);
        let clamped_right = x.saturating_add(radius).min(self.width - 1);
        let clamped_top = y.saturating_sub(radius);
        let clamped_bottom = y.saturating_add(radius).min(self.height - 1);

        if clamped_left > clamped_right || clamped_top > clamped_bottom {
            return;
        }

        for py in clamped_top..=clamped_bottom {
            let row = py * self.width;
            self.pixels[row + clamped_left..=row + clamped_right].fill(color);
        }
    }

    pub fn draw_point_normalized(&mut self, x: f64, y: f64, color: u32, size: usize) {
        let last_column = (self.width - 1) as f64;
        let last_row = (self.height - 1) as f64;

        let fraction_horizontal = (x + 1.0) / 2.0;
        let fraction_vertical = (1.0 - y) / 2.0;

        let screen_column = (fraction_horizontal * last_column).round();
        let screen_row = (fraction_vertical * last_row).round();

        if screen_column < 0.0 || screen_row < 0.0 {
            return;
        }

        self.draw_point(screen_column as usize, screen_row as usize, color, size);
    }

    pub fn draw_line(
        &mut self,
        start_column: f64,
        start_row: f64,
        end_column: f64,
        end_row: f64,
        color: u32,
        size: usize,
    ) {
        let horizontal_distance = end_column - start_column;
        let vertical_distance = end_row - start_row;

        let step_count = horizontal_distance
            .abs()
            .max(vertical_distance.abs())
            .round()
            .max(1.0) as usize;

        for step in 0..=step_count {
            let progress = step as f64 / step_count as f64;
            let column = (start_column + horizontal_distance * progress).round();
            let row = (start_row + vertical_distance * progress).round();

            if column >= 0.0 && row >= 0.0 {
                self.draw_point(column as usize, row as usize, color, size);
            }
        }
    }

    pub fn normalized_to_screen(&self, x: f64, y: f64) -> (f64, f64) {
        let last_column = (self.width - 1) as f64;
        let last_row = (self.height - 1) as f64;

        let fraction_horizontal = (x + 1.0) / 2.0;
        let fraction_vertical = (1.0 - y) / 2.0;

        (
            fraction_horizontal * last_column,
            fraction_vertical * last_row,
        )
    }
}

fn main() {
    let mut state = state::State {
        xz_angle: 0.0,
        xz_velocity: std::f64::consts::PI / 2.0,
        current_model_idx: 0,
        z_offset: 1.0,
        color: [Color::from_u32(0x0020FA45), Color::from_u32(0x00FA2045)],
        line_stroke_radius: 2,
        is_color_rotating: false,
    };

    let mut window = win32::Window::new("Simple 3D software render", WIDTH, HEIGHT, &mut state);

    let mut canvas = Canvas {
        width: WIDTH,
        height: HEIGHT,
        pixels: vec![0u32; WIDTH * HEIGHT],
    };

    let interval = Duration::from_micros(1000000 / FPS);
    let mut next_tick = Instant::now();
    let mut last_frame = Instant::now();

    while window.is_open() {
        let now = Instant::now();
        let delta_time = now - last_frame;
        last_frame = now;
        next_tick += interval;

        state.advance(delta_time.as_secs_f64());

        canvas.clear();

        let mut screen_positions = [(0.0, 0.0); PENGUIN.len()];

        // Render penguin

        if state.current_model_idx == 1 {
            for (index, point) in PENGUIN.into_iter().enumerate() {
                let mut rotated_point = point;
                rotated_point.rotate_xz(state.xz_angle);
                let (normalized_x, normalized_y) = rotated_point.project(state.z_offset);
                screen_positions[index] = canvas.normalized_to_screen(normalized_x, normalized_y);
            }

            for [first_corner, second_corner, third_corner] in PENGUIN_FACES {
                let triangle_edges = [
                    (first_corner, second_corner),
                    (second_corner, third_corner),
                    (third_corner, first_corner),
                ];

                for (start_index, end_index) in triangle_edges {
                    let (start_column, start_row) = screen_positions[start_index];
                    let (end_column, end_row) = screen_positions[end_index];
                    canvas.draw_line(
                        start_column,
                        start_row,
                        end_column,
                        end_row,
                        state.current_color_u32(),
                        state.line_stroke_radius,
                    );
                }
            }
        } else
        // Render cube
        {
            for (index, point) in CUBE.into_iter().enumerate() {
                let mut rotated_point = point;
                rotated_point.rotate_xz(state.xz_angle);
                rotated_point.project(state.z_offset);
                let (normalized_x, normalized_y) = rotated_point.project(state.z_offset);
                screen_positions[index] = canvas.normalized_to_screen(normalized_x, normalized_y);
            }

            for (start_index, end_index) in CUBE_EDGES {
                let (start_column, start_row) = screen_positions[start_index];
                let (end_column, end_row) = screen_positions[end_index];
                canvas.draw_line(
                    start_column,
                    start_row,
                    end_column,
                    end_row,
                    state.current_color_u32(),
                    state.line_stroke_radius,
                );
            }
        }

        window.update_with_buffer(canvas.pixels());

        let now = Instant::now();
        if next_tick > now {
            thread::sleep(next_tick - now)
        } else {
            next_tick = now;
        }
    }
}
