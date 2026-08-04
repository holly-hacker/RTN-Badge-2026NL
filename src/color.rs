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

    pub const fn new_w(val: u8) -> Self {
        Self::new(val, val, val)
    }

    pub const fn get_components(&self) -> [u8; 3] {
        [self.g, self.r, self.b]
    }

    pub const fn from_bool(value: bool) -> Color {
        if value { Color::WHITE } else { Color::OFF }
    }
}

// color constants
impl Color {
    #[allow(unused)]
    pub const OFF: Color = Color::new_w(0);
    #[allow(unused)]
    pub const WHITE: Color = Color::new_w(255);
}
