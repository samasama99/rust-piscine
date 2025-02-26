// create the following structures. Each has one element:
//
//     One: first_layer as type Option<Two>.
//     Two: second_layer as type Option<Three>
//     Three: third_layer as type Option<Four>
//     Four: fourth_layer as type Option<u16>.
//
// Beside the structures, you must create a function named get_fourth_layer, and associate it to the One structure. This function should return the Option value in the Four structure.
// Expected Function and structures
//


#[derive(Clone, Copy)]
pub struct One {
    pub first_layer: Option<Two>,
}
#[derive(Clone, Copy)]
pub struct Two {
    pub second_layer: Option<Three>,
}
#[derive(Clone, Copy)]
pub struct Three {
    pub third_layer: Option<Four>,
}
#[derive(Clone, Copy)]
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer?.second_layer?.third_layer?.fourth_layer
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
     let a = One {
         first_layer : Some(Two {
             second_layer: Some(Three {
                 third_layer: Some(Four {
                     fourth_layer: Some(1000)
                 })
             })
         })
     };

     assert_eq!(1000, match a.get_fourth_layer() {
         Some(e) => e,
         None => 0
     })
    }
}
