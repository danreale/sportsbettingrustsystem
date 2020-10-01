mod common;
use colored::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            common::help();
        },
        // one argument passed
        2 => {
            common::help();
        },
        3 => {
            common::help();
        },
        4 => {
            let odds1 = &args[1];
            let odds2 = &args[2];
            let bet = &args[3];

            let o1: i32 = odds1.parse().unwrap();
            let o2: i32 = odds2.parse().unwrap();
            let o3: f32 = bet.parse().unwrap();

            let fav: f32 = common::favorite_odds_juice(o2);
            let ud: f32 = common::favorite_odds_juice(o1);
            let total_odds: f32 = fav + ud;
            println!("Total Odds: {}{}", total_odds.to_string().yellow(), "%".yellow());
            let vig = common::get_juice(total_odds);
            println!("Vig: {}{}", vig.to_string().red().on_yellow().bold(), "%".red().on_yellow().bold());
            let vig_level = common::get_vig_level(vig);
            println!("Vig Level: {}", vig_level);
            common::get_real_odds(fav, ud);
            println!("Winnings Based on ${} Bet:", bet.to_string().cyan());
            let newfav: f32 = o2 as f32;
            let newud: f32 = o1 as f32;
            common::get_payouts(newud, newfav, o3);
        }
         // all the other cases
        _ => {
            common::help();
        }
    }
}
