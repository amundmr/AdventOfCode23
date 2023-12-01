use std::fs;
use std::collections::VecDeque;

pub fn day9() {
    println!("Day 9 solution:");
    let filename = "inputs/9.txt";
    println!("The input filename chosen is '{}'", filename);
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");
    //println!("{}", contents);
    let mut head_pos: (isize, isize) = (0, 0);
    let mut tail_pos_trace: Vec<(isize, isize)> = vec![(0, 0)]; // Adding the start point

    for line in contents.lines() {
        // Read in text file and move the head_pos (x,y) of the head accordningly
        let instruction: Vec<&str> = line.split_whitespace().collect();
        let dist: isize = instruction[1].parse::<isize>().unwrap();
        if instruction[0] == "U" {
            head_pos.1 += dist;
        } else if instruction[0] == "D" {
            head_pos.1 -= dist;
        } else if instruction[0] == "R" {
            head_pos.0 += dist;
        } else if instruction[0] == "L" {
            head_pos.0 -= dist;
        }

        // Check if tail is close enough to head
        let tail_pos: (isize, isize) = tail_pos_trace.last().unwrap().clone();

        if (head_pos.0 - tail_pos.0).abs() <= 1 && //Tail and head is within 1 horizontally
            (head_pos.1 - tail_pos.1) <= 1 { // Tail and head is within 1 vertically
            //println!("Tail is within dist");
            continue;
        }
        // If we get here, the tail isn't close enough.
        if head_pos.0 - tail_pos.0 > 1 && head_pos.1 == tail_pos.1 { // head is 2 right of tail,
                                                                     // but same hight
            tail_pos_trace.push((tail_pos.0+1, tail_pos.1));
        } else if head_pos.0 - tail_pos.0 < -1 && head_pos.1 == tail_pos.1 { //Head is 2 left of
                                                                             //tail but same height
            tail_pos_trace.push((tail_pos.0-1, tail_pos.1));
        } else if head_pos.0 == tail_pos.0 && head_pos.1 - tail_pos.1 > 1 { // head is 2 above tail
                                                                            // but same width
            tail_pos_trace.push((tail_pos.0, tail_pos.1+1));
        } else if head_pos.0 == tail_pos.0 && head_pos.1 - tail_pos.1 < -1 { // head is 2 below
                                                                             // tail but same width
            tail_pos_trace.push((tail_pos.0, tail_pos.1-1));
        } else if head_pos.0 - tail_pos.0 > 1 && head_pos.1 - tail_pos.1
    
    }
    for pos in tail_pos_trace.iter() {
        println!("x: {}, y: {}", pos.0, pos.1);
    }
    // They start at zero
    // Make a check: Is the tail close enough to the head?
    // If not, then move the tail to the new head_pos
    // If moved, push that (x,y) head_pos to a vec for reference.
    //
    // When that is done looping over every line in the input file, remove all duplicate elements
    // of the vector and take the length of it.
    // The output of that vector is the solution
}
