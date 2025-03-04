use crate::areas_volumes::{
    circle_area, cone_volume, cube_volume, parallelepiped_volume, rectangle_area, sphere_volume,
    square_area, triangle_area, triangular_pyramid_volume,
};
pub use crate::areas_volumes::{GeometricalShapes, GeometricalVolumes};
pub use std::f32::consts::PI;

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
        GeometricalShapes::Square => square_area(a) * times <= rect_area,
        GeometricalShapes::Circle => circle_area(a) as usize * times <= rect_area,
        GeometricalShapes::Rectangle => rectangle_area(a, b) * times <= rect_area,
        GeometricalShapes::Triangle => triangle_area(a, b) as usize * times <= rect_area,
    }
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let box_vol = x * y * z;
    match objects {
        GeometricalVolumes::Cube => cube_volume(a) as usize * times <= box_vol,
        GeometricalVolumes::Sphere => sphere_volume(a) as usize * times <= box_vol,
        GeometricalVolumes::Cone => cone_volume(a, b) as usize * times <= box_vol,
        GeometricalVolumes::Pyramid => triangular_pyramid_volume(a as f64, b) as usize * times <= box_vol,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) * times <= box_vol,
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
    }
    #[test]
    fn it_works_2() {
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
