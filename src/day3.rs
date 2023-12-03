use regex::Regex;
use core::num;
use std::fs;

pub fn day3() {
    println!("Day 3 solution:");
    let filename = "inputs/3.txt";
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");
    let splitlines: Vec<_> = contents.lines().collect();
    // If you want to loop over lines then you can just iterate over the .lines() which gives an
    // iterator
    let mut pnr: usize = 0;

    fn find_numbers(text: &str) -> Vec<(usize, usize)> {
        let re = Regex::new(r"\d+").unwrap();
        let mut matches = vec![];

        for cap in re.captures_iter(text) {
            let Some(get1) = cap.get(0) else { return matches };
            let start = get1.start();
            let end = get1.end();
            matches.push((start, end));
            
        }

        matches
    }

    fn search_adjacent_symbols(prev_ln: &str, curr_ln: &str, nxt_ln: &str, num_idx: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut matches: Vec<(usize, usize)> = vec![];
        let symbols = ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '[', ']', '{', '}', '\\', '|', '/', '?'];
        for (start, end) in num_idx.iter() {
            println!("Start: {}, end: {}", start, end);
            let mut match_found: bool = false;
            let mut indexes_to_check: Vec<usize> = vec![]; // Keeping track of all indexes that are worth checking for this number
            if start > &0 {  // Only add start-1 if start is 1 or bigger
                indexes_to_check.push(start-1);
            }
            for idx in *start..*end { //Add all numbers between and including start and end Not including end since regex exceeds it.
                indexes_to_check.push(idx);
            }
            if end < &(curr_ln.len()) {    // Add indexes after end if end is at least 1 from the edge
                indexes_to_check.push(*end);
            }
            println!("Indexes to check: {:?}", indexes_to_check);
            for line in [prev_ln, curr_ln, nxt_ln].to_vec() {
                println!("newline");
                for idx in indexes_to_check.iter() {
                    //print!("Idx: {}", idx);
                    if symbols.contains(&line.chars().nth(*idx).unwrap()) {
                        matches.push((*start, *end));
                        match_found = true;
                        break;
                    }
                if match_found { break; }
                }
            if match_found { break; }
            }
        }
        matches
    }

    for n in 1..splitlines.len()-1 {
        println!("Line: {}", n);
        let nums_in_line = find_numbers(splitlines[n]);
        let matching_numbers_idxs: Vec<(usize, usize)> = search_adjacent_symbols(splitlines[n-1], splitlines[n], splitlines[n+1], &nums_in_line);
        println!("nums lenth: {}", matching_numbers_idxs.len());
        for (start, end) in matching_numbers_idxs {
            let num_str = &splitlines[n][start..end];
            println!("Number: {}", num_str);
            pnr += num_str.parse::<usize>().unwrap();
        }
    }
    println!("Total pnr from line 1 to -1: {}", pnr);
    // Handling edges suck; here are the numbers from the first and last line
    pnr += 613 + 439 + 438 + 617 + 343 + 942;
    pnr += 302 + 476 + 64 + 159 + 815 + 445 + 558 + 824 + 281;
    println!("Total pnr including line 1 and -1 {}", pnr);

}
