use std::fs;
#[warn(dead_code)]
pub struct Lists {
    pub left: Vec<i32>,
    pub right: Vec<i32>,
}

#[warn(dead_code)]
struct Distances {
    distances: Vec<i32>,
}

pub fn parse_and_extract() -> Lists {
    let content = fs::read_to_string("src/input.txt").expect("not found");

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let left: i32 = parts[0].parse().expect("Failed to parse left number");
            let right: i32 = parts[1].parse().expect("Failed to parse right number");

            left_numbers.push(left);
            right_numbers.push(right);
        }
    }

    let mut numbers = Lists {
        left: left_numbers,
        right: right_numbers,
    };

    numbers
}

pub fn find_distances(mut numbers: Lists) -> Distances {
    let left_max = 99999;
    let right_max = 99999;

    let mut left_smallest = left_max;

    let mut right_smallest = right_max;

    let init_numbers_length = numbers.left.len();

    let mut distances_values = Vec::new();

    let mut i = 0;

    while i < init_numbers_length {
        let mut j = 0;
        let mut left_index = 0;
        let mut k = 0;
        let mut right_index = 0;

        for number in &numbers.left {
            if *number < left_smallest {
                left_smallest = *number;
                left_index = j;
            }
            j += 1;
        }
        for number in &numbers.right {
            if *number < right_smallest {
                right_smallest = *number;
                right_index = k;
            }
            k += 1;
        }
        numbers.left.remove(left_index);
        numbers.right.remove(right_index);

        distances_values.push((left_smallest - right_smallest).abs());

        left_smallest = left_max;
        right_smallest = right_max;

        i += 1;
    }

    let distacnes = Distances {
        distances: distances_values,
    };

    distacnes
}

pub fn add_distances(distances: Distances) -> i32 {
    let mut sum = 0;
    for value in distances.distances {
        sum += value;
    }

    sum
}
