use std::fs;
use std::collections::VecDeque;

pub fn day6() {
    println!("Day 6 solution:");
    let filename = "inputs/6.txt";
    println!("The input filename chosen is '{}'", filename);
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");
    println!("{}", contents);
    //let splitlines: Vec<_> = contents.lines().collect();
    //println!("{:?}", splitlines);
    
    //somewhere to store the last 4 chars
    let mut vec: VecDeque<char> = VecDeque::new();

    'outer: for (i, c) in contents.chars().enumerate() {
        vec.push_back(c);
        
        // Just to not do anything until length is 4. maybe there is a smoother way?
        if vec.len() < 4 {
            continue
        }
        let mut clone1 = vec.clone();
        let mut clone2 = vec.clone();
        for thing in clone1.iter() {
            println!("Clone 1: {}", thing);
        }

        // Issue with this is that when j and k are both first element, they are ofcourse equal, so
        // I need a better way of checking this
        for j in clone1.iter() {
            for k in clone2.iter() {
                if j == k {
                    // then not unique, remove front element and continue
                    vec.pop_front();
                    continue 'outer;
                }
            }
        }
        println!("Wow, we didn't find an equal element! We are on iteration {}+1", i);
    break


    }

}
