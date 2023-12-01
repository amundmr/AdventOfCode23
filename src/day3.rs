use std::fs;

pub fn day3() {
    println!("Day 3 solution:");
    let filename = "inputs/3.txt";
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");
    let splitlines: Vec<_> = contents.lines().collect();
    // If you want to loop over lines then you can just iterate over the .lines() which gives an
    // iterator
    let mut pts: i32 = 0;

    for n in 0..splitlines.len() {
        let opt: Vec<&str> = splitlines[n].split_whitespace().collect();
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

}
