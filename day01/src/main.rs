use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
    let mut a_list: Vec<i32> = Vec::new();
    let mut b_list: Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut map: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines("./files/data.txt") {
        for line in lines.flatten() {
            let numbers: Vec<&str> = line.split("   ").collect();
            a_list.push(numbers[0].trim().parse().unwrap());
            b_list.push(numbers[1].trim().parse().unwrap());
        }

        // 1st puzzle implementation
        // a_list.sort();
        // b_list.sort();

        // for i in 0..a_list.len() {
        //     sum += i32::abs(a_list[i] - b_list[i]);
        // }

        // println!("total sum of difference: {sum}");

        for i in 0..b_list.len() {
            match map.get(&b_list[i]) {
                Some(count) => {
                    map.insert(b_list[i], count + 1);
                }
                None => {
                    map.insert(b_list[i], 1);
                }
            }
        }

        for i in 0..a_list.len() {
            match map.get(&a_list[i]) {
                Some(count) => {
                    sum += a_list[i] * count;
                }
                None => {
                    
                }
            }
        }

        println!("total sum of reps: {sum}");
    } else {
        println!("error: unable to parse numbers.");
    }
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
