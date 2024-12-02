use crate::first_part;

pub struct Similarity {
    pub sim: Vec<i32>,
}

pub fn find_similarity(numbers: first_part::Lists) -> Similarity {
    let num_length = numbers.right.len();
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    let mut left_num = 0;

    let mut values = Vec::new();

    while i < num_length {
        for number in &numbers.right {
            left_num = numbers.left[j];
            if numbers.left[j] == *number {
                count += 1;
            }
        }

        values.push(left_num * count);

        count = 0;

        j += 1;
        i += 1;
    }

    let similarities = Similarity { sim: values };

    similarities
}

pub fn get_similaarity_value(values: Similarity) -> i32 {
    let mut sum = 0;

    for value in values.sim {
        sum += value;
    }

    sum
}
