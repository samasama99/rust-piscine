#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if self.r == first && self.g == second {
            self.r = second;
            self.g = first;
        } else if self.r == second && self.g == first {
            self.r = first;
            self.g = second;
        } else if self.r == first && self.b == second {
            self.r = second;
            self.b = first;
        } else if self.r == second && self.b == first {
            self.r = first;
            self.b = second;
        } else if self.r == first && self.a == second {
            self.r = second;
            self.a = first;
        } else if self.r == second && self.a == first {
            self.r = first;
            self.a = second;
        } else if self.g == first && self.b == second {
            self.g = second;
            self.b = first;
        } else if self.g == second && self.b == first {
            self.g = first;
            self.b = second;
        } else if self.g == first && self.a == second {
            self.g = second;
            self.a = first;
        } else if self.g == second && self.a == first {
            self.g = first;
            self.a = second;
        } else if self.b == first && self.a == second {
            self.b = second;
            self.a = first;
        } else if self.b == second && self.a == first {
            self.b = first;
            self.a = second;
        }

        self
    }
}
