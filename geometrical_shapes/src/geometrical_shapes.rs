// Instructions
//
// The purpose of this exercise is to create an image like the example below:
//
// example
//
// You will need to do the following:
//
// Copy the code in the usage to your main.rs.
//
// Create a module called geometrical_shapes in another file.
//
// You'll define the logic for creating and working with shapes in geometrical_shapes. Create the following traits:
//
// Drawable which contains the methods draw and color.
//
// Displayable which contains the method display.
//
// Define them according to the way they are called in the main.rs function.
//
// In order to compile and run main.rs, you'll need to define some structures.
// You are free to implement all the shapes with whatever internal structure you see fit,
// but you must provide an associated function new for all the shapes, which will be described below:
//
// Point: a new point should be created from two i32 values.
// Line: a new line should be created from references to two different points.
// Triangle: a new triangle should be created from references to three different points.
// Rectangle: a new rectangle should be created from references to two different points.
// Circle: a new circle should be created from a reference to a point representing the center, and an i32 value representing the circle's radius.
//
// You'll also need to create the associated function random for Line, Point and Circle. You should derive their signatures from the usage.

use rand::Rng;
use raster::{Color, Image};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Line(pub(crate) Point, pub(crate) Point);

impl Line {
    pub(crate) fn random(h: i32, w: i32) -> Self {
        let mut rng = rand::rng();
        let x1 = rng.random_range(0..w);
        let x2 = rng.random_range(0..w);
        let y1 = rng.random_range(0..h);
        let y2 = rng.random_range(0..h);
        let p1 = Point(x1, x2);
        let p2 = Point(y1, y2);
        if x1 == x2 && y1 == y2 {
            Self::random(h, w)
        } else {
            Self(p1, p2)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Point(pub(crate) i32, pub(crate) i32);

impl Point {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

impl Point {
    pub(crate) fn random(h: i32, w: i32) -> Self {
        let mut rng = rand::rng();
        let x = rng.random_range(0..w);
        let y = rng.random_range(0..h);
        if x == y {
            Self::random(h, w)
        } else {
            Self(x, y)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Rectangle(pub(crate) Point, pub(crate) Point);

impl Rectangle {
    pub(crate) fn new(p1: Point, p2: Point) -> Self {
        Self(p1, p2)
    }
}

pub(crate) struct Triangle(pub(crate) Point, pub(crate) Point, pub(crate) Point);

impl Triangle {
    pub(crate) fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Self(p1, p2, p3)
    }
}

pub(crate) struct Circle(pub(crate) Point, pub(crate) i32);

impl Circle {
    pub(crate) fn random(h: i32, w: i32) -> Self {
        let mut rng = rand::rng();
        let x = rng.random_range(0..w);
        let y = rng.random_range(0..h);
        let p = Point(x, y);
        let r = rng.random_range(1..(h + w) / 2);

        Self(p, r)
    }
}
pub(crate) trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
pub(crate) trait Drawable {
    fn draw(&self, image: &mut Image);
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let p1 = self.0;
        let p2 = self.1;
        let p3 = self.2;
        Line(p1, p2).draw(image);
        Line(p2, p3).draw(image);
        Line(p3, p1).draw(image);
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        let h = image.height;
        let w = image.height;
        let Point(x, y) = *self;

        if x >= w || y >= h || x < 0 || y < 0 {
            return;
        }

        let mut rng = rand::rng();
        let r = rng.random_range(0..255);
        let g = rng.random_range(0..255);
        let b = rng.random_range(0..255);
        if let Err(err) = image.set_pixel(x, y, Color::rgb(r, g, b)) {
            println!("Error: draw point failed cause: {err:#?}")
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let Line(Point(x1, y1), Point(x2, y2)) = *self;

        let dx = x2 - x1;
        let dy = y2 - y1;
        let n = dx.abs().max(dy.abs());
        let dt: f64 = n as f64;
        let dxdt: f64 = dx as f64 / dt;
        let dydt: f64 = dy as f64 / dt;

        let mut x = x1 as f64;
        let mut y = y1 as f64;
        for _ in 0..n {
            Point(x.round() as i32, y.round() as i32).draw(image);
            x += dxdt;
            y += dydt;
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let p1 = self.0;
        let p2 = self.1;
        let p3 = Point(p2.0, p1.1);
        let p4 = Point(p1.0, p2.1);

        Line(p1, p3).draw(image);
        Line(p1, p4).draw(image);
        Line(p2, p4).draw(image);
        Line(p2, p3).draw(image);
    }
}
impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let r = self.1;
        let mut x = r;
        let mut y = 0;
        let x_centre = self.0 .0;
        let y_centre = self.0 .1;

        if (r > 0) {
            Point(x + x_centre, -y + y_centre).draw(image);
            Point(y + x_centre, x + y_centre).draw(image);
            Point(-y + x_centre, x + y_centre).draw(image);
        }

        let mut p = 1 - r;

        while (x > y) {
            y += 1;

            if p <= 0 {
                p = p + 2 * y + 1;
            } else {
                x -= 1;
                p = p + 2 * y - 2 * x + 1;
            }

            if x < y {
                break;
            }

            Point(x + x_centre, y + y_centre).draw(image);
            Point(-x + x_centre, y + y_centre).draw(image);
            Point(x + x_centre, -y + y_centre).draw(image);
            Point(-x + x_centre, -y + y_centre).draw(image);

            if (x != y) {
                Point(y + x_centre, x + y_centre).draw(image);
                Point(-y + x_centre, x + y_centre).draw(image);
                Point(y + x_centre, -x + y_centre).draw(image);
                Point(-y + x_centre, -x + y_centre).draw(image);
            }
        }
    }
}
