use json::object;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

fn convert_to_kcal(calories: Vec<String>) -> Vec<f64> {
    let mut kcal_values: Vec<f64> = Vec::new();

    for cal in calories {
        if cal.contains("kJ") {
            let value: f64 = cal.trim_end_matches("kJ").parse().unwrap();
            let kcal = value / 4.18;
            kcal_values.push(kcal);
        } else if cal.contains("kcal") {
            let value: f64 = cal.trim_end_matches("kcal").parse().unwrap();
            kcal_values.push(0f64);
        }
    }

    kcal_values
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let res = foods.into_iter().fold(object! {
                "cals": 0,
                "carbs": 0,
                "proteins": 0,
                "fats": 0
    },|acc, food| {
       object! {
           "cals" : acc["cals"].as_f64().unwrap() + convert_to_kcal(food.calories.to_vec()).iter().sum::<f64>() * food.nbr_of_portions,
           "carbs" : acc["carbs"].as_f64().unwrap() + food.carbs * food.nbr_of_portions,
           "proteins" : acc["proteins"].as_f64().unwrap() + food.proteins * food.nbr_of_portions,
           "fats" : acc["fats"].as_f64().unwrap() + food.fats * food.nbr_of_portions,
       }
    });

    object! {
        "cals" : format!("{:.2}", res["cals"].as_f64().unwrap()).parse::<f64>().unwrap(),
        "carbs" : format!("{:.2}", res["carbs"].as_f64().unwrap()).parse::<f64>().unwrap(),
        "proteins" : format!("{:.2}", res["proteins"].as_f64().unwrap()).parse::<f64>().unwrap(),
        "fats" : format!("{:.2}", res["fats"].as_f64().unwrap()).parse::<f64>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let res = calculate_macros(a);
      assert_eq!(
         res["cals"].as_f32().unwrap(),
          2777.39
      );

        assert_eq!(
            res["carbs"].as_f32().unwrap(),
            322.44
        );
        assert_eq!(
            res["proteins"].as_f32().unwrap(),
            122.06
        );
        assert_eq!(
             res["fats"].as_f32().unwrap(),
            106.93
        );
    }
}
