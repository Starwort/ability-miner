use std::env::args;

use ability_miner::{get_results, Brand, Slot};

fn main() {
    let mut args = args().peekable();
    args.next().unwrap();
    let gear_brand = args.next().unwrap().parse::<Brand>().unwrap();
    let mut slots = Vec::new();
    while let Some(ability) = args.next() {
        let mut slot = Slot {
            ability: ability
                .parse()
                .unwrap_or_else(|_| panic!("{ability} not a valid ability")),
            drink: None,
        };
        if args.peek().map(|s| &**s) == Some("drink") {
            args.next().unwrap();
            slot.drink = Some(args.next().unwrap().parse().unwrap());
        }
        slots.push(slot);
    }
    for seed in get_results(0..=u32::MAX, Some(100), gear_brand, &slots) {
        println!("{seed:10} {seed:08x}");
    }
}
