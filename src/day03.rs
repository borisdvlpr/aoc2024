use crate::helpers::read_lines;

use regex::Regex;

pub fn run() {
    let re_mul = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let re_enable = Regex::new(r"(?i).*?(don't|do)\(\)").unwrap();
    let mut enabled = true;
    let mut result = 0;

    if let Ok(lines) = read_lines("./files/day03/data.txt") {
        let lines: Vec<String> = lines.flatten().collect();
        for line in &lines {
            for mult in re_mul.find_iter(&line) {
                let mut pos = 0;

                for temp in re_enable.find_iter(&line[pos..mult.start()]) {
                    enabled = temp.as_str().to_lowercase().contains("do()");
                }

                if enabled {
                    let val = mult.as_str().replace("mul(", "").replace(")", "");
                    let num: Vec<&str> = val.split(",").collect();
                    result += num[0].parse::<i32>().unwrap() * num[1].parse::<i32>().unwrap();
                }

                pos = mult.end();
            }
        }

        println!("result is: {}", result);
    } else {
        println!("error: unable to read data file.");
    }
}
