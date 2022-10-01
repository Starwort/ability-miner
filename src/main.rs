use std::env::args;
use std::fmt::Display;
use std::process::exit;
use std::sync::atomic::{AtomicU32, Ordering};

use ability_miner::{get_ability, Ability, Brand};
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

struct Slot {
    ability: Ability,
    drink: Option<Ability>,
}

fn main() {
    let mut args = args().peekable();
    args.next().unwrap();
    let gear_brand = args.next().unwrap().parse::<Brand>().unwrap();
    let mut slots = Vec::new();
    while let Some(ability) = args.next() {
        let mut slot = Slot {
            ability: ability.parse().unwrap(),
            drink: None,
        };
        if args.peek().map(|s| &**s) == Some("drink") {
            args.next().unwrap();
            slot.drink = Some(args.next().unwrap().parse().unwrap());
        }
        slots.push(slot);
    }
    let results = (0..=u32::MAX)
        .into_par_iter()
        .filter(|seed| slots_match(*seed, &gear_brand, &slots));
    let count = AtomicU32::new(0);
    results.for_each(|result| {
        if count.load(Ordering::Relaxed) < 100 {
            println!("Possible seed: {result}");
            count.fetch_add(1, Ordering::Relaxed);
        } else {
            exit(0)
        }
    })
}

fn slots_match(mut seed: u32, &gear_brand: &Brand, slots: &[Slot]) -> bool {
    for Slot {
        ability,
        drink,
    } in slots
    {
        if get_ability(&mut seed, gear_brand, *drink) != *ability {
            return false;
        }
    }
    true
}
