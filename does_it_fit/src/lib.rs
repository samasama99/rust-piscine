use crate::areas_volumes::{GeometricalShapes, GeometricalVolumes};
use std::f32::consts::PI;

pub mod areas_volumes;

// Create area_fit.
// It should return true if the geometric shape can fit inside the rectangular area as many times
//      as is indicated by times.
//
// The arguments of the function will be:
//
// x and y: length and width of the rectangular area.
// objects: the type of geometric shape.
// times: the number of geometric shapes to fit inside the rectangular area.
// a: size of dimension for:
//      - side of a Square
//      - base of a Triangle
//      - radius of a Circle
//      - side_a of a Rectangle
// b: size of dimension for:
//      - height of a Triangle
//      - side_b of a Rectangle

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let rect_area = x * y;
    match objects {
        GeometricalShapes::Square => (a * a) * times <= rect_area,
        GeometricalShapes::Circle => (PI * a.pow(2) as f32) as usize * times <= rect_area,
        GeometricalShapes::Rectangle => a * b * times <= rect_area,
        GeometricalShapes::Triangle => (a * b / 2) * times <= rect_area,
    }
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let box_vol = x * y * z;
    match objects {
        GeometricalVolumes::Cube => a.pow(3) * times <= box_vol,
        GeometricalVolumes::Sphere => {
            (4.0 / 3.0 * PI) as usize * a.pow(3) * times <= box_vol
        }
        GeometricalVolumes::Cone => {
            (1.0 / 3.0 * PI) as usize * a * b.pow(2) * times <= box_vol
        }
        GeometricalVolumes::Pyramid => {
            (a * b * c * 1 / 3 ) * times <= box_vol
        }
        GeometricalVolumes::Parallelepiped => {
            a * b * c * times <= box_vol
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;
    #[test]
    fn it_works() {
        assert!(area_fit(2, 4, GeometricalShapes::Rectangle, 100, 2, 1).not());
        assert!(area_fit(5, 5, GeometricalShapes::Triangle, 3, 5, 3));
        assert!(volume_fit(5, 5, 5, GeometricalVolumes::Sphere, 3, 2, 0, 0));
        assert!(volume_fit(
            5,
            7,
            5,
            GeometricalVolumes::Parallelepiped,
            1,
            6,
            7,
            4
        ));
    }
}

//
// }
//
// And its output:
//
// $ cargo run
// Do 100 rectangles (2x1) fit in a 2 by 4 square? false
// Do 3 triangles (5 base and 3 height) fit in a 5 by 5 square? true
// Do 3 spheres (2 radius) fit in a 5 by 5 by 5 box? true
// Does 1 parallelepiped (6 base, 7 height and depth 4) fit in a 5 by 7 by 5 parallelepiped? true
