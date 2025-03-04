use json::object;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f32,
    pub fats: f32,
    pub carbs: f32,
    pub nbr_of_portions: f32,
}

fn convert_to_kcal(calories: Vec<String>) -> Vec<f32> {
    let mut kcal_values: Vec<f32> = Vec::new();

    for cal in calories {
        if cal.contains("kJ") {
            let value : f32 = cal.trim_end_matches("kJ").parse().unwrap();
            let kcal = value / 4.184;
            kcal_values.push(kcal);
        } else if cal.contains("kcal") {
            let value = cal.trim_end_matches("kcal").parse().unwrap();
            kcal_values.push(value);
        }
    }

    kcal_values
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    foods.into_iter().fold(object! {
                "cals": 0,
                "carbs": 0,
                "proteins": 0,
                "fats": 0
    },|acc, food| {
       object! {
           "cals" : acc["proteins"].as_f32().unwrap() + convert_to_kcal(food.calories.to_vec()).iter().sum::<f32>() * food.nbr_of_portions,
           "carbs" : acc["carbs"].as_f32().unwrap() + food.carbs * food.nbr_of_portions,
           "proteins" : acc["proteins"].as_f32().unwrap() + food.proteins * food.nbr_of_portions,
           "fats" : acc["fats"].as_f32().unwrap() + food.fats * food.nbr_of_portions,
       }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use json::object;

    #[test]
    fn it_works() {
        let a = vec![
            Food {
                name: String::from("big mac"),
                calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
                proteins: 27.0,
                fats: 26.0,
                carbs: 41.0,
                nbr_of_portions: 2.0,
            },
            Food {
                name: "pizza margherita".to_string(),
                calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ];

        assert_eq!(
            object! {
                "cals": 2777.39,
                "carbs": 322.44,
                "proteins": 122.06,
                "fats": 106.93
            },
            calculate_macros(a)
        );
    }
}
