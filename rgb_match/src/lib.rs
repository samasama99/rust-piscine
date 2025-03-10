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

    pub fn swap(mut self, first: char, second: char) -> Color {
        match (first, second) {
            ('r', 'g') | ('g', 'r') => {
                let temp = self.r;
                self.r = self.g;
                self.g = temp;
            }
            ('r', 'b') | ('b', 'r') => {
                let temp = self.r;
                self.r = self.b;
                self.b = temp;
            }
            ('r', 'a') | ('a', 'r') => {
                let temp = self.r;
                self.r = self.a;
                self.a = temp;
            }
            ('g', 'b') | ('b', 'g') => {
                let temp = self.g;
                self.g = self.b;
                self.b = temp;
            }
            ('g', 'a') | ('a', 'g') => {
                let temp = self.g;
                self.g = self.a;
                self.a = temp;
            }
            ('b', 'a') | ('a', 'b') => {
                let temp = self.b;
                self.b = self.a;
                self.a = temp;
            }
            _ => panic!("Invalid component"),
        }
        self
    }
}
