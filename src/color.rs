#[macro_export]
macro_rules! argb_alpha {
    ($val:expr) => {{
        let validated: u32 = $val;
        ((validated >> 24) & 0xFF) as u8
    }};
}

#[macro_export]
macro_rules! argb_red {
    ($val:expr) => {{
        let validated: u32 = $val;
        ((validated >> 16) & 0xFF) as u8
    }};
}

#[macro_export]
macro_rules! argb_green {
    ($val:expr) => {{
        let validated: u32 = $val;
        ((validated >> 8) & 0xFF) as u8
    }};
}

#[macro_export]
macro_rules! argb_blue {
    ($val:expr) => {{
        let validated: u32 = $val;
        ((validated) & 0xFF) as u8
    }};
}

pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn parse(argb_string: &str) -> Self {
        println!("Color::parse is not implemented yet.");
        Self {
            a: 0,
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn from_u32(argb: u32) -> Self {
        Self {
            a: argb_alpha!(argb),
            r: argb_red!(argb),
            g: argb_green!(argb),
            b: argb_blue!(argb),
        }
    }

    pub fn shift_hue_approx(&mut self, degrees: f64) {
        let rad = degrees.to_radians();
        let cos_theta = rad.cos();
        let sin_theta = rad.sin();

        println!("Shifting: rad {rad}, cos {cos_theta}, sin {sin_theta}");

        let red = self.r as f64;
        let green = self.g as f64;
        let blue = self.b as f64;

        let new_red = red * (0.213 + 0.787 * cos_theta + 0.213 * sin_theta)
            + green * (0.715 - 0.715 * cos_theta - 0.715 * sin_theta)
            + blue * (0.072 - 0.072 * cos_theta + 0.928 * sin_theta);

        let new_green = red * (0.213 - 0.213 * cos_theta + 0.143 * sin_theta)
            + green * (0.715 + 0.285 * cos_theta + 0.140 * sin_theta)
            + blue * (0.072 - 0.072 * cos_theta - 0.283 * sin_theta);

        let new_blue = red * (0.213 - 0.213 * cos_theta - 0.787 * sin_theta)
            + green * (0.715 - 0.715 * cos_theta + 0.715 * sin_theta)
            + blue * (0.072 + 0.928 * cos_theta + 0.072 * sin_theta);

        self.r = new_red.round().clamp(0.0, 255.0) as u8;
        self.g = new_green.round().clamp(0.0, 255.0) as u8;
        self.b = new_blue.round().clamp(0.0, 255.0) as u8;
    }

    pub fn shift_hue(&mut self, degrees: f64) {
        let fraction_red = (self.r as f64) / 255.0;
        let fraction_green = (self.g as f64) / 255.0;
        let fraction_blue = (self.b as f64) / 255.0;

        let color_max = fraction_red.max(fraction_green).max(fraction_blue);
        let color_min = fraction_red.min(fraction_green).min(fraction_blue);

        let delta = color_max - color_min;

        let saturation = if color_max == 0.0 {
            0.0
        } else {
            delta / color_max
        };

        let hue = (if delta == 0.0 {
            0.0
        } else if color_max == fraction_red {
            60.0 * ((fraction_green - fraction_blue) / delta).rem_euclid(6.0)
        } else if color_max == fraction_green {
            60.0 * ((fraction_blue - fraction_red) / delta + 2.0)
        } else {
            60.0 * ((fraction_red - fraction_green) / delta + 4.0)
        });

        let hue = (hue + degrees).rem_euclid(360.0);

        let chroma = color_max * saturation;
        let h1 = hue / 60.0;
        let x = chroma * (1.0 - ((h1 % 2.0) - 1.0).abs());
        let m = color_max - chroma;

        let out_r: f64;
        let out_g: f64;
        let out_b: f64;

        if h1 < 1.0 {
            out_r = chroma;
            out_g = x;
            out_b = 0.0;
        } else if h1 < 2.0 {
            out_r = x;
            out_g = chroma;
            out_b = 0.0;
        } else if h1 < 3.0 {
            out_r = 0.0;
            out_g = chroma;
            out_b = x;
        } else if h1 < 4.0 {
            out_r = 0.0;
            out_g = x;
            out_b = chroma;
        } else if h1 < 5.0 {
            out_r = x;
            out_g = 0.0;
            out_b = chroma;
        } else {
            out_r = chroma;
            out_g = 0.0;
            out_b = x;
        }

        self.r = ((out_r + m) * 255.0).round() as u8;
        self.g = ((out_g + m) * 255.0).round() as u8;
        self.b = ((out_b + m) * 255.0).round() as u8;
    }

    pub fn as_u32_argb(&self) -> u32 {
        let alpha = (self.a as u32) << 24;
        let red = (self.r as u32) << 16;
        let green = (self.g as u32) << 8;
        let blue = self.b as u32;

        alpha | red | green | blue
    }

    pub fn set_u32(&mut self, argb: u32) {
        self.a = argb_alpha!(argb);
        self.r = argb_red!(argb);
        self.g = argb_green!(argb);
        self.b = argb_blue!(argb);
    }

    pub fn add_u32(&mut self, argb: u32) {
        let alpha = argb_alpha!(argb);
        let red = argb_red!(argb);
        let green = argb_green!(argb);
        let blue = argb_blue!(argb);

        self.a = self.a.saturating_add(alpha);
        self.r = self.r.saturating_add(red);
        self.g = self.g.saturating_add(green);
        self.b = self.b.saturating_add(blue);
    }
    pub fn sub_u32(&mut self, argb: u32) {
        let alpha = argb_alpha!(argb);
        let red = argb_red!(argb);
        let green = argb_green!(argb);
        let blue = argb_blue!(argb);

        self.a = self.a.saturating_sub(alpha);
        self.r = self.r.saturating_sub(red);
        self.g = self.g.saturating_sub(green);
        self.b = self.b.saturating_sub(blue);
    }
}
