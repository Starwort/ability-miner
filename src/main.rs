use std::env::args;
use std::string::ParseError;

use ability_miner::*;

fn main() {
    let mut seed = args()
        .next()
        .expect("Missing seed")
        .parse::<u32>()
        .expect("Not a valid seed");
    let brand = args()
        .next()
        .expect("Missing brand")
        .parse::<Brand>()
        .expect("Not a valid brand");
    let drink = args()
        .next()
        .map(|ability| ability.parse().expect("not a valid ability"));
    for _ in 0..10 {
        println!("{seed}, {}", get_ability(&mut seed, brand, drink))
    }
    println!("{seed}");
}
