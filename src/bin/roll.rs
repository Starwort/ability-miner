use std::fmt::Display;

use ability_miner::*;
use clap::Parser;

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

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// The gear seed
    #[arg(short, long)]
    seed: u32,
    /// The gear brand, by internal name
    #[arg(short, long)]
    brand: Brand,
    /// The drink you currently have active, if any
    #[arg(short, long, default_value_t = MyOption::<Ability>::None)]
    drink: MyOption<Ability>,
    /// How many abilities to generate
    #[arg(short, long, default_value_t = 1)]
    times: usize,
}

fn main() {
    let Args {
        mut seed,
        brand,
        drink,
        times,
    } = Args::parse();
    let drink = drink.to_option();
    for _ in 0..times {
        let ability = get_ability(&mut seed, brand, drink);
        println!("{seed}, {ability}");
    }
}
