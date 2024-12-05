use crate::helpers::read_lines;

pub fn run() {
    let mut sum: u16 = 0;

    if let Ok(lines) = read_lines("./files/day02/data.txt") {
        for line in lines.flatten() {
            let numbers: Vec<&str> = line.split(" ").collect();

            if check_sequence(&numbers, true) || check_sequence(&numbers, false) {
                sum += 1;
            }
        }

        println!("safe reports: {sum}");
    } else {
        println!("error: unable to read data file.");
    }
}

fn check_sequence(numbers: &[&str], ascending: bool) -> bool {
    if numbers.len() < 2 {
        return false;
    }

    for i in 0..numbers.len() {
        let filtered_numbers: Vec<&str> = numbers
            .iter()
            .enumerate()
            .filter(|&(index, _)| index != i)
            .map(|(_, &num)| num)
            .collect();

        if filtered_numbers.windows(2).all(|n| {
            let num1: i32 = n[0].trim().parse().unwrap();
            let num2: i32 = n[1].trim().parse().unwrap();
            let diff: i32 = i32::abs(num1 - num2);

            if ascending {
                num1 < num2 && diff >= 1 && diff <= 3
            } else {
                num1 > num2 && diff >= 1 && diff <= 3
            }
        }) {
            return true;
        }
    }

    false
}
