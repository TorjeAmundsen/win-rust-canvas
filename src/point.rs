use crate::DISTANCE_FROM_SCREEN;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn rotate_xz(&mut self, angle: f64) {
        let cos: f64 = angle.cos();
        let sin: f64 = angle.sin();
        let x = self.x;
        self.x = x * cos - self.z * sin;
        self.z = x * sin + self.z * cos;
    }

    pub fn project(&self) -> (f64, f64) {
        (
            self.x / (self.z + DISTANCE_FROM_SCREEN),
            self.y / (self.z + DISTANCE_FROM_SCREEN),
        )
    }
}
