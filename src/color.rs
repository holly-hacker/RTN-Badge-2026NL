pub struct Color {
    // NOTE: order should probably be optimized?
    g: u8,
    r: u8,
    b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn get_components(&self) -> [u8; 3] {
        [self.g, self.r, self.b]
    }

    pub const fn from_bool(value: bool) -> Color {
        if value { Color::WHITE_HALF } else { Color::OFF }
    }

    pub const fn from_bool_dim(value: bool) -> Color {
        if value {
            Color::WHITE_SIXTEENTH
        } else {
            Color::OFF
        }
    }
}

// color constants
impl Color {
    #[allow(unused)]
    pub const OFF: Color = Color::new(0, 0, 0);
    #[allow(unused)]
    pub const WHITE: Color = Color::new(255, 255, 255);

    #[allow(unused)]
    pub const WHITE_HALF: Color = Color::new(127, 127, 127);
    #[allow(unused)]
    pub const WHITE_QUARTER: Color = Color::new(63, 63, 63);
    #[allow(unused)]
    pub const WHITE_EIGHT: Color = Color::new(31, 31, 31);
    #[allow(unused)]
    pub const WHITE_SIXTEENTH: Color = Color::new(15, 15, 15);
}
