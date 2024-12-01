use std::fs;

pub fn locations() {

    let location_ids = fs::read_to_string("resources/locations.txt")
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

    let mut previous_index_part_2 = -1;
    let mut part_1_answer = 0;

    for ids_1 in ids_part1.iter() {

        let mut actual_index_part_2 = 0;

        for ids_2 in ids_part2.iter() {

            if actual_index_part_2 <= previous_index_part_2 {

                actual_index_part_2 += 1;
                continue;
            }

            if ids_1 > ids_2 {

                part_1_answer += ids_1 - ids_2;

                previous_index_part_2 = actual_index_part_2;
                break;
            }
            else if ids_2 > ids_1 {

                part_1_answer += ids_2 - ids_1;

                previous_index_part_2 = actual_index_part_2;
                break;
            }
            else if ids_2 == ids_1 {

                part_1_answer += 0;

                previous_index_part_2 = actual_index_part_2;
                break;
            }
        }
    }

    let part_2_answer = part2(ids_part1, ids_part2);

    println!("total: {:?}", part_1_answer);
    println!("total: {:?}", part_2_answer);
}

fn part2(ids_part1: Vec<i32>, ids_part2: Vec<i32>) -> i32 {

    let mut similarity_scores: Vec<i32> = Vec::new();

    for ids_1 in ids_part1.iter() {

        let mut similarity_score = 0;

        for ids_2 in ids_part2.iter() {

            if ids_1 == ids_2 {

                similarity_score += 1;
            }
        }

        similarity_scores.push(ids_1 * similarity_score);
    }

    let mut similarities_total_sum = 0;

    for score in similarity_scores.iter() {

        similarities_total_sum += score;
    }

    similarities_total_sum
}

