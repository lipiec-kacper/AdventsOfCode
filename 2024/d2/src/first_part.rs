#![allow(dead_code, unused)]
use std::fs;

pub struct Reports {
    reports: Vec<Vec<i32>>,
}

//31 34 36 38 40 43 46 44
pub fn parse_input() -> Reports {
    let content = fs::read_to_string("src/input.txt").expect("not found");

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut numbers: Vec<i32> = Vec::new();

        for numbers_str in parts {
            let number: i32 = numbers_str.parse().expect("Failed to parse left number");
            numbers.push(number);
        }

        reports.push(numbers);
    }

    // for report in &reports {
    //     println!("{:?}", report[0]);
    // }

    let mut data_rep = Reports { reports: reports };

    return data_rep;
}

pub fn algo(lists: Reports) -> i32 {
    let mut pass: bool = false;

    let mut count: i32 = 0;

    let mut is_inc: i32 = 0;

    for line in &lists.reports {
        println!("{:?}", line);
        for i in 0..line.len() - 1 {
            if (line[i] - line[i + 1]) > 0
                && (line[i] - line[i + 1]) <= 3
                && (is_inc == 1 || is_inc == 0)
            {
                println!("{:?} - {:?} passes the test", line[i], line[i + 1]);
                is_inc = 1;
                pass = true;
                continue;
            } else if (line[i] - line[i + 1]) < 0
                && (line[i] - line[i + 1]).abs() <= 3
                && (line[i] - line[i + 1]).abs() >= 1
                && (is_inc == -1 || is_inc == 0)
            {
                println!("{:?} - {:?} passes the test", line[i], line[i + 1]);
                is_inc = -1;
                pass = true;
                continue;
            } else {
                pass = false;
                break;
            }
        }
        is_inc = 0;

        println!("");
        if pass == true {
            println!("passed");
            count += 1;
        }
    }

    return count;
}

//TODO: fix increasing and decreasing
