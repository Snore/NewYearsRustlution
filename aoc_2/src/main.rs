use std::fs;
use std::env;
use regex::Regex;

struct GamePull {
    red: u32,
    green: u32,
    blue: u32
}

impl GamePull {
    // fn new(red: u32, 
    //        green: u32, 
    //        blue: u32) -> GamePull {
    //     return GamePull{ red, green, blue };
    // }

    fn parse( rec: &str ) -> GamePull {
        let re = Regex::new(r"\d+").unwrap();
        let mut all_red: u32 = 0;
        let mut all_green: u32 = 0;
        let mut all_blue: u32 = 0;
        for game_rec in rec.split(",") {
            let x: u32 = re.find(game_rec).unwrap().as_str().parse().expect("oops");
            if game_rec.contains("red") {
                all_red += x;
            }
            else if game_rec.contains("green") {
                all_green += x;
            }
            else if game_rec.contains("blue") {
                all_blue += x;
            }
            else {
                panic!("Parsing bad color!");
            }
        }
        return GamePull{ red: all_red, 
                         green: all_green, 
                         blue: all_blue};
    }

    fn is_possible( &self,
                    red_pull: u32, 
                    green_pull: u32, 
                    blue_pull: u32 ) -> bool {
        return ( self.red <= red_pull ) && 
               ( self.green <= green_pull ) && 
               ( self.blue <= blue_pull );
    }

    fn get_power( &self ) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    number: u32,
    pulls: Vec<GamePull>
}

impl Game {
    // fn new( pulls: Vec<GamePull> ) -> Game {
    //     return Game{ pulls };
    // }

    fn new( number: u32, record: &str ) -> Game {
        let pulls: Vec<GamePull> = record.split(";").map(|rec: &str| GamePull::parse(rec)).collect();
        return Game{ number, 
                     pulls };
    }

    fn get_min_needed_cubes( &self ) -> GamePull {
        // find max r, g, b of all pulls
        let max_red: u32 = self.pulls.iter().max_by_key(|gp| gp.red ).unwrap().red;
        let max_green: u32 = self.pulls.iter().max_by_key(|gp| gp.green ).unwrap().green;
        let max_blue: u32 = self.pulls.iter().max_by_key(|gp| gp.blue ).unwrap().blue;

        GamePull { red: max_red, green: max_green, blue: max_blue }
    }
}

fn parse_record( record: &str ) -> Game {
    let pull_delim_pos: usize = record.find(":").unwrap();
    let game_number: u32 = record[(record.find("Game ").unwrap() + "Game ".len())..pull_delim_pos].parse().expect("Not a number");
    return Game::new(game_number, &record[(pull_delim_pos+1)..]);
}

fn load_games( game_records: &str ) -> Vec<Game> {
    let games: Vec<Game> = game_records.split("\n").map(|record: &str| parse_record(record)).collect();

    games
}

fn main() {
    // get input
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    // read
    let contents: String = fs::read_to_string(file_path).unwrap();
    println!("{contents}");

    // load
    let games: Vec<Game> = load_games(&contents);

    let red_truth: u32 = 12;
    let green_truth: u32 = 13;
    let blue_truth: u32 = 14;

    let ans_part1 = games.iter()
                              .filter(|g| g.pulls.iter().all(|p| p.is_possible(red_truth, green_truth, blue_truth)))
                              .fold(0, |acc, g| acc + g.number);

    println!("Your answer for part 1 is [{ans_part1}]");

    let ans_part2: u32 = games.iter()
                              .map(|g| g.get_min_needed_cubes())
                              .map(|gp| gp.get_power())
                              .fold(0, |acc, ans| acc + ans);
    println!("Your answer for part 2 is [{ans_part2}]");
}
