use std::fs;

pub fn day1() {
    println!("Day 1 solution:");
    let filename = "inputs/1.txt";
    println!("The input filename chosen is '{}'", filename);
    let contents =
        fs::read_to_string(filename).expect("Something went wrong when reading the file.");
    //println!("{}", contents);
    let splitlines: Vec<_> = contents.lines().collect();
    //println!("{:?}", splitlines);

    // Summing numbers, break at newline, continue
    let mut calsum: i32 = 0;
    let mut vec = Vec::new();

    for n in 1..splitlines.len() {
        if splitlines[n] == ""{
            vec.push(calsum);
            //println!("Found NEwline! Calsum: {}", calsum);
            calsum = 0;
            continue;
        }

        let cal: i32 = splitlines[n].parse::<i32>().unwrap();
        calsum += cal;

    }
    let maxval = vec.iter().max().unwrap();
    println!("Max calorie count is: {}", maxval);

    // Part TWO!
    vec.sort();
    vec.reverse();
    let top3sum: i32 = vec[0]+vec[1]+vec[2];
    println!("The top 3 summed calories are: {}", top3sum);


}
