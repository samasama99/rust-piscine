// Imagine you are working on some software to control smart lights in a house.
//      You have access to an array of all the lights in that house.
//
// Define the associated function new, and add it to the data structure Light.
//      It should create a new light with the alias passed as an argument, with a brightness of 0.
//
// Define the function change_brightness,
//      which receives a slice of lights,
//      an alias and a u8value.
//      It should attempt to find the correct light by its alias, and change the value of the brightness if found.

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_owned(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    lights
        .into_iter()
        .filter(|l| l.alias == alias)
        .for_each(|l| l.brightness = value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);
        assert_eq!(0, lights[0].brightness);
        change_brightness(&mut lights, "living_room", 200);
        assert_eq!(200, lights[0].brightness);
    }
}
