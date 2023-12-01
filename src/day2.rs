use std::fs;

pub fn day2() {
    println!("Day 2 solution:");
    let filename = "inputs/2.txt";
    println!("The input filename chosen is '{}'", filename);
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");
    //println!("{}", contents);
    let splitlines: Vec<_> = contents.lines().collect();
    //println!("{:?}", splitlines);

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
        
        /*
        Winning combinations
        "A":"Y",
        "B":"Z",
        "C":"X"
           
        Draw combinations:
        "A":"X",
        "B":"Y",
        "C":"Z"

        Loss combinations: All else
*/

        let round_pts: i32 = me_pts + win_pts;
        println!("me_pts={}, win_pts = {}, round_pts={}",me_pts, win_pts, round_pts);
        pts += round_pts;

    }
    //let mut maxval: i32 = 1;
    println!("My total points count is: {}", pts);
    // Part TWO!
    let mut pts2: i32 = 0;

    for n in 0..splitlines.len() {
        let opt: Vec<&str> = splitlines[n].split_whitespace().collect();
        println!("Opponent: {}, Me: {}", opt[0], opt[1]);
        
        let win_pts:i32 = match opt[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0
        };

        let me_pts:i32 = match (opt[0], opt[1]) {
            ("A", "X") => 3, //scirrors (3) loses to rock(A) and cause a loss (X)
            ("A", "Y") => 1,
            ("A", "Z") => 2,
            ("B", "X") => 1,
            ("B", "Y") => 2,
            ("B", "Z") => 3,
            ("C", "X") => 2,
            ("C", "Y") => 3,
            ("C", "Z") => 1,
            _ => 0
        };
        pts2 += win_pts;
        pts2 += me_pts;

    }
    println!("Part 2 pts: {}", pts2);

}
