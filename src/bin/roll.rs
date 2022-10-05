use std::fmt::Display;
use std::str::FromStr;

use ability_miner::*;
use clap::Parser;

#[derive(Debug, Clone, Copy)]
enum MyOption<T> {
    Some(T),
    None,
}
impl<'a, T> From<&'a str> for MyOption<T>
where
    T: FromStr,
{
    fn from(val: &'a str) -> Self {
        match T::from_str(val) {
            Ok(val) => Self::Some(val),
            Err(_) => Self::None,
        }
    }
}
impl<T> MyOption<T> {
    pub fn into_option(self) -> Option<T> {
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
    brand: MyOption<Brand>,
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
    let drink = drink.into_option();
    for _ in 0..times {
        let ability = get_ability(
            &mut seed,
            brand.into_option().expect("Invalid brand provided"),
            drink,
        );
        println!("{seed:10} {seed:08x}, {ability}");
    }
}
