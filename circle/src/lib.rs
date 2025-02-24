// Create the structures Circle and Point. You'll need to create the necessary methods for the code to compile correctly.
// Methods:
//     Point:
//         distance() -> returns the distance between two coordinates.
//     Circle:
//         diameter() -> returns the diameter of the circle.
//         area() -> returns the area of the circle.
//         intersect() -> returns if two circles intersect.
//
// Associated Functions
//
//     Circle:
//         new() -> receives three 64-bit floating point numbers in the following order: x, y and radius (x and y are the coordinates of the center of the new circle).
//                                                              The function returns a new circle.
//

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Circle {
            center: Point(x, y),
            radius: r,
        }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    // π × (diam/2)2
    pub fn area(&self) -> f64 {
        PI  * (self.diameter() / 2.0).powi(2)
    }

    pub fn intersect(&self, c: Circle) -> bool {
        self.center.distance(c.center) <= self.radius + c.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    // d=√((x2 – x1)² + (y2 – y1)²)
    pub fn distance(&self, p: Point) -> f64 {
        ((self.1 - p.1).powi(2) + (self.1 - p.1).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {
            center: Point(80.0, 115.0),
            radius: 30.0,
        };
        let point_a = Point(1.0, 1.0);
        let point_b = Point(0.0, 0.0);
        assert_eq!(70685.83470577035, circle.area());
        assert_eq!(300., circle.diameter());
        assert_eq!(60., circle1.diameter());
        assert_eq!(false, circle.intersect(circle1));

        assert_eq!(1.4142135623730951, point_a.distance(point_b));
    }
}
