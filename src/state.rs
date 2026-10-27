pub struct State {
    pub xz_angle: f64,
    pub xz_velocity: f64,
    pub current_model_idx: usize,
    pub z_offset: f64,
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
    }

    pub fn move_away_from_screen(&mut self) {
        self.z_offset += 0.1;
    }

    pub fn move_towards_screen(&mut self) {
        self.z_offset -= 0.1;
    }
}
