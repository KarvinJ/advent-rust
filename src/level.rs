use std::fs;

pub fn run() {

    let levels = fs::read_to_string("resources/test.txt")
        .expect("file not found");

    let mut safe_list: Vec<&str> = Vec::new();
    let mut safe_levels_quantity = 0;

    let is_part2 = false;

    levels.lines().for_each(|ids| {

        let mut is_safe = true;

        let mut level_index = 0;

        let levels_arrays: Vec<&str> = ids.split_whitespace().collect();

        let mut previous_level = levels_arrays[0].parse::<i32>().unwrap();
        let second_level = levels_arrays[1].parse::<i32>().unwrap();

        let is_ascending = previous_level < second_level;

        let mut unsafe_levels_quantity = 0;

        for level in levels_arrays.iter() {

            let actual_level = level.parse::<i32>().unwrap();

            if level_index == 0 {

                level_index += 1;
                previous_level = actual_level;
                continue;
            }

            level_index += 1;

            let mut level_gap = previous_level - actual_level;

            if is_ascending && actual_level > previous_level && level_gap.abs() < 4 {

                previous_level = actual_level;
            }
            else if !is_ascending && actual_level < previous_level && level_gap.abs() < 4 {

                previous_level = actual_level;
            }
            else if is_part2 {

                unsafe_levels_quantity += 1;
                previous_level = actual_level;

                if unsafe_levels_quantity > 1 {

                    is_safe = false;
                    break;
                }
            }
            else {
                is_safe = false;
                break;
            }
        }

        if is_safe {

            safe_levels_quantity += 1;
            safe_list.push("safe")
        }
        else {
            safe_list.push("unsafe")
        }
    });

    println!("Part 1 answer: {:?}", safe_list);
    println!("Part 1 answer: {:?}", safe_levels_quantity);
    // println!("Part 2 answer: {:?}", part_2_answer);
}


