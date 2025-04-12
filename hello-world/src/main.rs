use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2025-04-01"));
}
