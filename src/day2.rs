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

    let mut game_sum: u32 = 0;
    let mut power: u32 = 0;
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    for n in 0..splitlines.len() {
         let mut possible_game: bool = true;

        // Get game ID
        let game_vec: Vec<&str> = splitlines[n].split(":").collect();
        let game_id_vec:Vec<&str> = game_vec[0].split(" ").collect();
        let game_id: u32 = game_id_vec[1].parse::<u32>().unwrap();
        println!("game_id: {}", game_id);

        // Loop over the sets in the game
        let sets_vec: Vec<&str> = game_vec[1].split(";").collect();
        let mut game_max_red: u32 = 0;
        let mut game_max_green: u32 = 0;
        let mut game_max_blue: u32 = 0;
        for set in sets_vec.iter() {
            // Loop over the cubes in the set
            let cube_vec: Vec<&str> = set.split(",").collect();
            println!("-");

            let mut red_num: u32 = 0;
            let mut green_num: u32 = 0;
            let mut blue_num: u32 = 0;
            for cube in cube_vec.iter() {
                // Check if the cube strin contains
                let cube_clean = cube.trim();
                println!("{}", cube_clean);
                if cube_clean.contains("red") {
                    red_num = cube_clean.split(" ").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
                    println!("red_num: {}", red_num);
                    if red_num > game_max_red {
                        game_max_red = red_num;
                    }
                } else if cube_clean.contains("green") {
                    green_num= cube_clean.split(" ").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
                    println!("green_num: {}", green_num);
                    if green_num > game_max_green {
                        game_max_green = green_num;
                    }
                } else if cube_clean.contains("blue") {
                    blue_num = cube_clean.split(" ").collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
                    println!("blue_num: {}", blue_num);
                    if blue_num > game_max_blue {
                        game_max_blue = blue_num;
                    }
                }
            }

            if red_num > max_red  || green_num > max_green || blue_num > max_blue{
                possible_game = false
            }
            
        }
        if possible_game {
            game_sum += game_id;
        }
        let game_power: u32 = game_max_red*game_max_green*game_max_blue;
        power += game_power;
    }
    println!("Sum of possible game IDs: {}", game_sum);
    println!("Total power: {}", power);
}
