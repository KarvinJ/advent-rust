use std::fs;

pub fn locations() {

    let location_ids = fs::read_to_string("resources/test.txt")
        .expect("file not found");

    let mut ids_part1: Vec<i32> = Vec::new();
    let mut ids_part2: Vec<i32> = Vec::new();

    location_ids.lines().for_each(|ids| {

        let id_arrays: Vec<&str> = ids.split_whitespace().collect();

        let numeric_ids_1 = id_arrays[0].parse::<i32>().unwrap();
        let numeric_ids_2 = id_arrays[1].parse::<i32>().unwrap();

        ids_part1.push(numeric_ids_1);
        ids_part2.push(numeric_ids_2);
    });

    ids_part1.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ids_part2.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut previous_elements: Vec<i32> = Vec::new();
    let mut sum: Vec<i32> = Vec::new();

    let mut previous_index_part_2 = -1;
    let mut total = 0;

    for ids_1 in ids_part1.iter() {

        let mut actual_index_part_2 = 0;

        for ids_2 in ids_part2.iter() {

            if previous_index_part_2 == actual_index_part_2 {
                continue;
            }

            if ids_1 > ids_2 {

                total += ids_1 - ids_2;
                sum.push(ids_1 - ids_2);

                previous_index_part_2 += 1;
                break;
            }
            else if ids_2 > ids_1 {

                total += ids_2 - ids_1;
                sum.push(ids_2 - ids_1);

                previous_index_part_2 += 1;
                break;
            }
        }
    }


    println!("total: {:?}", sum);
    println!("previous index: {:?}", previous_elements);
}

