use std::fs;

pub fn day5() {
    println!("Day 5 solution:");
    let filename = "inputs/5.txt";
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");
    let splitlines: Vec<_> = contents.lines().collect();
    // If you want to loop over lines then you can just iterate over the .lines() which gives an
    // iterator
    //println!("Readout of contents: {}", contents);

    // Interpret stacks and put them as strings in vectors
    let mut stack1: Vec<char> = Vec::new();
    let stacklines:usize = 8; //line 0 to 7
    /*
    for n in 0..stacklines {
        let ch1 = splitlines[n].chars().nth(1).unwrap();
        let ch2 = splitlines[n].chars().nth(5).unwrap();
        let ch3 = splitlines[n].chars().nth(9).unwrap();
        let ch4 = splitlines[n].chars().nth(13).unwrap();
        let ch5 = splitlines[n].chars().nth(17).unwrap();
        let ch6 = splitlines[n].chars().nth(21).unwrap();
        let ch7 = splitlines[n].chars().nth(25).unwrap();
        let ch8 = splitlines[n].chars().nth(29).unwrap();
        let ch9 = splitlines[n].chars().nth(33).unwrap();
        println!("Chars: {}, {}, {}, {}, {}, {}, {}, {}, {}", ch1, ch2, ch3, ch4, ch5, ch6, ch7, ch8, ch9);
        stack1.push(ch1);
        stack2.push(ch2);
        stack3.push(ch3);
        stack4.push(ch4);
        stack5.push(ch5);
        stack6.push(ch6);
        stack7.push(ch7);
        stack8.push(ch8);
        stack9.push(ch9);
    }
    */
    // Fuck it, lets insert manually
    let mut stacks: Vec<Vec<&str>> = Vec::new();
    stacks.push(vec!["Z", "P", "M", "H", "R"]);
    stacks.push(vec!["P", "C", "J", "B"]);
    stacks.push(vec!["S", "N", "H", "G", "L", "C", "D"]);
    stacks.push(vec!["F", "T", "M", "D", "Q", "S", "R", "L"]);
    stacks.push(vec!["F", "S", "P", "Q", "B", "T", "Z", "M"]);
    stacks.push(vec!["T", "F", "S", "Z", "B", "G"]);
    stacks.push(vec!["N", "R", "V"]);
    stacks.push(vec!["P", "G", "L", "T", "D", "V", "C", "M"]);
    stacks.push(vec!["W", "Q", "N", "J", "F", "M", "L"]);
    for ch in stacks[0].iter() {
        println!("stack1: {}", ch);
    
    }
    let mut stacks9001 = stacks.clone();

    // Get the rest of the text and move the stuff
    for n in 10..splitlines.len() {
        let linevec: Vec<_> = splitlines[n].split_whitespace().collect();
        let amount: usize = linevec[1].parse::<usize>().unwrap();
        let from_stack: usize = linevec[3].parse::<usize>().unwrap() - 1;
        let to_stack: usize = linevec[5].parse::<usize>().unwrap() - 1;
        //println!("amount: {}, from: {}, to: {}", amount, from_stack, to_stack);
        
        for _n in 0..amount {
            let transfer: &str = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(transfer);
        }

    }
    for stack in stacks.iter() {
        for crat in stack.iter() {
            print!("{}", crat);
        }
        println!("");
    }
    
    // Part two
    println!("Part two!");
    for n in 10..splitlines.len() {
        let linevec: Vec<_> = splitlines[n].split_whitespace().collect();
        let amount: usize = linevec[1].parse::<usize>().unwrap();
        let from_stack: usize = linevec[3].parse::<usize>().unwrap()-1;
        let to_stack: usize = linevec[5].parse::<usize>().unwrap()-1;
        //println!("amount: {}, from: {}, to: {}", amount, from_stack, to_stack);
        
        let from_stack_height: usize = stacks9001[from_stack].len() - amount;
        let mut transfer: Vec<&str> = stacks9001[from_stack].split_off(from_stack_height);
        stacks9001[to_stack].append(&mut transfer);

    }
    for stack in stacks9001.iter() {
        for crat in stack.iter() {
            print!("{}", crat);
        }
        println!("");
    }


    /*
    for n in 0..splitlines.len() {
        let opt.push(splitlines[n].split_whitespace().collect();
        println!("Opponent: {}, Me: {}", opt[0], opt[1]);

        
        let me_pts:i32 = match opt[1] {
            "X" => 1, //Rock
            "Y" => 2, //Paper
            "Z" => 3, //Scissors
            _ => 0
        };
        let win_pts:i32 = match (opt[0], opt[1]) {
            ("A", "Y") => 6,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("A", "X") => 3,
            ("B", "Y") => 3,
            ("C", "Z") => 3,
            _ => 0
        };
        let round_pts: i32 = me_pts + win_pts;
        println!("me_pts={}, win_pts = {}, round_pts={}",me_pts, win_pts, round_pts);
        pts += round_pts;

    }
    println!("My total points count is: {}", pts);
    */
}
