use std::fs;
use std::collections::HashMap;
use std::iter::Iterator;

pub fn day1() {
    println!("Day 1 solution:");
    let filename = "inputs/1.txt";
    //println!("The input filename chosen is '{}'", filename);
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");
    //println!("{}", contents);
    let splitlines: Vec<_> = contents.lines().collect();
    //println!("{:?}", splitlines);

    // Summing numbers, break at newline, continue
    let mut calsum: u32 = 0;

    for n in 0..splitlines.len() {
        let mut first_dig: char = '0';
        let mut last_dig: char = '0';
        for chr in splitlines[n].chars() {
            if chr.is_digit(10) {
                first_dig = chr;
                break;
            }
        }
        for chr in splitlines[n].chars().rev() {
            if chr.is_digit(10) {
                last_dig = chr;
                break;
            }
        }

        //println!("{}{}", first_dig, last_dig);
        let num = format!("{}{}", first_dig, last_dig).parse::<u32>().unwrap();
        
        calsum += num;
    }
    println!("{}", calsum);

    // Part 2
    const NUMERALS: [(&str, u32); 20] = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut calsum2: u32 = 0;
    for line in splitlines.iter(){
        let mut numbers = Vec::new();
        for curr in 0..line.len() {
            for (numeral, value) in NUMERALS.iter() {
                if line[curr..].starts_with(numeral) {
                    numbers.push(value);
                    break;
                }
            }
        }
        let first = *numbers.first().unwrap();
        let last = *numbers.last().unwrap();
        let solution2 = (first * 10) + last;
        calsum2 += solution2;    
    }   
    println!("{}", calsum2);

}
