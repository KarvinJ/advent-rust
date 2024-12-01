use std::fs;

pub fn calories_calculator() {

    let calories = fs::read_to_string("resources/calories.txt")
        .expect("file not found");

    let mut actual_calories_sum = 0;
    let mut max_calories_elf = 0;

    let mut top_calories_elf = Vec::new();

    calories.lines().for_each(|calorie| {

        if calorie.is_empty() {

            if actual_calories_sum > max_calories_elf {

                max_calories_elf = actual_calories_sum;
                top_calories_elf.push(max_calories_elf);
            }

            actual_calories_sum = 0;
        }
        else {

            actual_calories_sum += calorie.parse::<i32>().unwrap();
        }
    });

    let total_elements = top_calories_elf.len();

    let total_calories_of_top3 = top_calories_elf.get(total_elements-1).unwrap() + top_calories_elf.get(total_elements - 2).unwrap() + top_calories_elf.get(total_elements - 3).unwrap();

    println!("Max calories elf: {}", max_calories_elf);
    println!("Total calories of top 3: {}", total_calories_of_top3);
}