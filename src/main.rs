use std::env::args;
use std::fmt::Display;
use std::process::exit;
use std::sync::atomic::{AtomicU32, Ordering};

use ability_miner::{get_ability, get_results, Ability, Brand, Slot};
use clap::Parser;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone, Copy)]
enum MyOption<T> {
    Some(T),
    None,
}
impl<'a, T> From<&'a str> for MyOption<T>
where
    T: From<&'a str>,
{
    fn from(val: &'a str) -> Self {
        MyOption::Some(T::from(val))
    }
}
impl<T> MyOption<T> {
    pub fn to_option(self) -> Option<T> {
        match self {
            Self::Some(val) => Some(val),
            Self::None => None,
        }
    }
}
impl<T> Display for MyOption<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Some(val) => write!(f, "{val}"),
            Self::None => write!(f, "<nothing>"),
        }
    }
}

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
    for seed in get_results(100, gear_brand, &slots) {
        println!("{seed:10} {seed:08x}");
    }
}
