pub struct State {
    pub xz_angle: f64,
    pub xz_velocity: f64,
    pub current_model_idx: usize,
    pub z_offset: f64,
    pub color: [u32; 2],
    pub line_stroke_radius: usize,
}

impl State {
    pub fn advance(&mut self, delta_time: f64) {
        self.xz_angle += self.xz_velocity * delta_time;
    }

    pub fn reverse_spin_direction(&mut self) {
        self.xz_velocity = -self.xz_velocity;
    }

    pub fn switch_model(&mut self) {
        self.current_model_idx = (self.current_model_idx + 1) % 2;
        self.print_color();
    }

    pub fn move_away_from_screen(&mut self) {
        self.z_offset += 0.1;
    }

    pub fn move_towards_screen(&mut self) {
        self.z_offset -= 0.1;
        self.z_offset = self.z_offset.max(0.6);
    }

    pub fn increase_red(&mut self) {
        let mut red = (self.color[self.current_model_idx] & 0x00FF0000) >> 16;
        red = (red + 5).min(255);
        let cleared = self.color[self.current_model_idx] & !0x00FF0000;
        let shifted = (red & 0xFF) << 16;
        self.color[self.current_model_idx] = cleared | shifted;
        self.print_color();
    }

    pub fn decrease_red(&mut self) {
        let mut red = (self.color[self.current_model_idx] & 0x00FF0000) >> 16;
        red = red.wrapping_sub(5);
        if red > 255 {
            red = 0;
        }
        let cleared = self.color[self.current_model_idx] & !0x00FF0000;
        let shifted = (red & 0xFF) << 16;
        self.color[self.current_model_idx] = cleared | shifted;
        self.print_color();
    }

    pub fn increase_green(&mut self) {
        let mut green = (self.color[self.current_model_idx] & 0x0000FF00) >> 8;
        green = (green + 5).min(255);
        let cleared = self.color[self.current_model_idx] & !0x0000FF00;
        let shifted = (green & 0xFF) << 8;
        self.color[self.current_model_idx] = cleared | shifted;
        self.print_color();
    }

    pub fn decrease_green(&mut self) {
        let mut green = (self.color[self.current_model_idx] & 0x0000FF00) >> 8;
        green = green.wrapping_sub(5);
        if green > 255 {
            green = 0;
        }
        let cleared = self.color[self.current_model_idx] & !0x0000FF00;
        let shifted = (green & 0xFF) << 8;
        self.color[self.current_model_idx] = cleared | shifted;
        self.print_color();
    }

    pub fn increase_blue(&mut self) {
        let mut blue = self.color[self.current_model_idx] & 0x000000FF;
        blue = (blue + 5).min(255);
        let cleared = self.color[self.current_model_idx] & !0x000000FF;
        let shifted = blue & 0xFF;
        self.color[self.current_model_idx] = cleared | shifted;
        self.print_color();
    }

    pub fn decrease_blue(&mut self) {
        let mut blue = self.color[self.current_model_idx] & 0x000000FF;
        blue = blue.wrapping_sub(5);
        if blue > 255 {
            blue = 0;
        }
        let cleared = self.color[self.current_model_idx] & !0x000000FF;
        let shifted = blue & 0xFF;
        self.color[self.current_model_idx] = cleared | shifted;
        self.print_color();
    }

    pub fn set_line_stroke_radius(&mut self, stroke_size: usize) {
        if stroke_size == 0 {
            return;
        }

        self.line_stroke_radius = stroke_size;
    }

    fn print_color(&self) {
        println!("RGB: {:06x}", self.color[self.current_model_idx]);
    }
}
