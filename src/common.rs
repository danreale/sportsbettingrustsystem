use colored::*;

pub fn favorite_odds_juice(odds: i32) -> f32{
  let odds_num: f32 = odds as f32 * -1.0;
  let fav_juice: f32;
  if odds < 0 {
    fav_juice = odds_num / (odds_num + 100.0) * 100.0;
    println!("Favorite Odds: {}", odds.to_string().on_white().bright_red().bold());
    println!("Favorite Odds Percentage with Vig: {}{}", fav_juice.to_string().red(), "%".red());
  } else {
    let new_odds_num: f32 = odds as f32;
    fav_juice = 100.0 / (new_odds_num + 100.0) * 100.0;
    println!("Underdog Odds: {}{}", "+".on_white().bright_red().bold(), odds.to_string().on_white().bright_red().bold());
    println!("Underdog Odds Percentage with Vig: {}{}", fav_juice.to_string().red(), "%".red());
  }
  return fav_juice;
}
pub fn get_juice(odds: f32) -> f32{
  let vig: f32 = odds - 100.0;
  return vig;
}
pub fn get_vig_level(vig: f32) -> String {
  let mut level: String = String::from("");
  if vig > 5.0 {
    level = String::from("High").cyan().bold().to_string();
  }
  else if vig >= 1.0 && vig <= 5.0 {
    level = String::from("Normal").cyan().bold().to_string();
  }
  else if vig < 1.0 {
    level = String::from("Low").cyan().bold().to_string();
  }
  return level;
}
pub fn get_real_odds(fav: f32, ud: f32) {
  let favorite: f32 = fav / (fav + ud) * 100.0;
  let underdog: f32 = ud / (fav + ud) * 100.0;
  println!("Favorite Real Odds: {}{}", favorite.to_string().red(), "%".red());
  println!("Underdog Real Odds: {}{}", underdog.to_string().red(), "%".red());
}
pub fn help() {
  println!("{}{}{}{}{}{}", "Need to Pass in Underdog and Favorite\n".yellow(),
"usage:
cargo run <underdog> <favorite>\n".cyan(),
"The underdog is the positive number and the favorite is the negative number\n".yellow(),
"example:
cargo run ".cyan(), "+250 ".red().bold(), "-260".red().bold());
}