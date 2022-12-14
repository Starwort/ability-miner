#![feature(const_for, const_mut_refs, trait_alias)]
use std::fmt::Display;
use std::str::FromStr;
#[cfg(feature = "rayon")]
use std::sync::atomic::{AtomicU32, Ordering};
#[cfg(feature = "rayon")]
use std::sync::Mutex;

#[cfg(feature = "rayon")]
use rayon::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
pub use wasm_bindgen_rayon::init_thread_pool;

pub const fn advance_seed(seed: &mut u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, enum_utils::FromStr, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ability {
    /// Ink Saver (Main)
    MainInk_Save = 0,
    /// Ink Saver (Sub)
    SubInk_Save,
    /// Ink Recovery Up
    InkRecovery_Up,
    /// Run Speed Up
    HumanMove_Up,
    /// Swim Speed Up
    SquidMove_Up,
    /// Special Charge Up
    SpecialIncrease_Up,
    /// Special Saver
    RespawnSpecialGauge_Save,
    /// Special Power Up
    SpecialSpec_Up,
    /// Quick Respawn
    RespawnTime_Save,
    /// Quick Super Jump
    JumpTime_Save,
    /// Sub Power Up
    SubSpec_Up,
    /// Ink Resistance Up
    OpInkEffect_Reduction,
    /// Sub Resistance Up
    SubEffect_Reduction,
    /// Intensify Action
    Action_Up,
}
impl Ability {
    #[must_use]
    pub const fn from_usize(val: usize) -> Self {
        #[allow(clippy::enum_glob_use)]
        use Ability::*;
        [
            MainInk_Save,
            SubInk_Save,
            InkRecovery_Up,
            HumanMove_Up,
            SquidMove_Up,
            SpecialIncrease_Up,
            RespawnSpecialGauge_Save,
            SpecialSpec_Up,
            RespawnTime_Save,
            JumpTime_Save,
            SubSpec_Up,
            OpInkEffect_Reduction,
            SubEffect_Reduction,
            Action_Up,
        ][val]
    }

    #[must_use]
    pub const fn internal_name(&self) -> &'static str {
        [
            "MainInk_Save",
            "SubInk_Save",
            "InkRecovery_Up",
            "HumanMove_Up",
            "SquidMove_Up",
            "SpecialIncrease_Up",
            "RespawnSpecialGauge_Save",
            "SpecialSpec_Up",
            "RespawnTime_Save",
            "JumpTime_Save",
            "SubSpec_Up",
            "OpInkEffect_Reduction",
            "SubEffect_Reduction",
            "Action_Up",
        ][*self as usize]
    }

    #[must_use]
    pub const fn legacy_internal_name(&self) -> &'static str {
        [
            "MainInk_Save",
            "SubInk_Save",
            "InkRecovery_Up",
            "HumanMove_Up",
            "SquidMove_Up",
            "SpecialIncrease_Up",
            "RespawnSpecialGauge_Save",
            "SpecialTime_Up",
            "RespawnTime_Save",
            "JumpTime_Save",
            "BombDistance_Up",
            "OpInkEffect_Reduction",
            "SubEffect_Reduction",
            "Action_Up",
        ][*self as usize]
    }
}
impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(clippy::enum_glob_use)]
        use Ability::*;
        match self {
            MainInk_Save => write!(f, "Ink Saver (Main)"),
            SubInk_Save => write!(f, "Ink Saver (Sub)"),
            InkRecovery_Up => write!(f, "Ink Recovery Up"),
            HumanMove_Up => write!(f, "Run Speed Up"),
            SquidMove_Up => write!(f, "Swim Speed Up"),
            SpecialIncrease_Up => write!(f, "Special Charge Up"),
            RespawnSpecialGauge_Save => write!(f, "Special Saver"),
            SpecialSpec_Up => write!(f, "Special Power Up"),
            RespawnTime_Save => write!(f, "Quick Respawn"),
            JumpTime_Save => write!(f, "Quick Super Jump"),
            SubSpec_Up => write!(f, "Sub Power Up"),
            OpInkEffect_Reduction => write!(f, "Ink Resistance Up"),
            SubEffect_Reduction => write!(f, "Sub Resistance Up"),
            Action_Up => write!(f, "Intensify Action"),
        }
    }
}
impl From<&str> for Ability {
    fn from(val: &str) -> Self {
        Self::from_str(val).unwrap()
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, enum_utils::FromStr, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Brand {
    /// SquidForce
    B00 = 0,
    /// Zink
    B01,
    /// Krak-On
    B02,
    /// Rockenberg
    B03,
    /// Zekko
    B04,
    /// Forge
    B05,
    /// Firefin
    B06,
    /// Skalop
    B07,
    /// Splash Mob
    B08,
    /// Inkline
    B09,
    /// Tentatek
    B10,
    /// Takoroka
    B11,
    /// Annaki
    B15,
    /// Enperry
    B16,
    /// Toni Kensa
    B17,
    /// Unused, but still had brand data
    B18,
    /// Barazushi
    B19,
    /// Emberz
    B20,
    /// Grizzco
    B97,
    /// Cuttlegear
    B98,
    /// amiibo
    B99,
    None,
}
impl Brand {
    pub const DRINK_RESULT: [[[Ability; 34]; 22]; 14] = {
        let mut result = [[[Ability::from_usize(0); 34]; 22]; 14];
        let mut drink = 0;
        while drink < 14 {
            let mut brand = 0;
            while brand < 22 {
                let mut ability = 0;
                let mut position = 0;
                while ability < 14 {
                    if ability != drink {
                        let mut times = 0;
                        while times < Self::_WEIGHTS[brand][ability] {
                            result[drink][brand][position] =
                                Ability::from_usize(ability);
                            position += 1;
                            times += 1;
                        }
                    }
                    ability += 1;
                }
                brand += 1;
            }
            drink += 1;
        }
        result
    };
    pub const NO_DRINK_RESULT: [[Ability; 35]; 22] = {
        let mut result = [[Ability::from_usize(0); 35]; 22];
        let mut brand = 0;
        while brand < 22 {
            let mut ability = 0;
            let mut position = 0;
            while ability < 14 {
                let mut times = 0;
                while times < Self::_WEIGHTS[brand][ability] {
                    result[brand][position] = Ability::from_usize(ability);
                    position += 1;
                    times += 1;
                }
                ability += 1;
            }
            brand += 1;
        }
        result
    };
    const _WEIGHTS: [BrandData; 22] = {
        #[allow(clippy::enum_glob_use)]
        use Ability::*;
        [
            Self::weights(MainInk_Save, OpInkEffect_Reduction),
            Self::weights(RespawnTime_Save, JumpTime_Save),
            Self::weights(SubEffect_Reduction, SquidMove_Up),
            Self::weights(SquidMove_Up, HumanMove_Up),
            Self::weights(SpecialIncrease_Up, RespawnSpecialGauge_Save),
            Self::weights(SubInk_Save, SpecialSpec_Up),
            Self::weights(InkRecovery_Up, SubInk_Save),
            Self::weights(RespawnSpecialGauge_Save, RespawnTime_Save),
            Self::weights(HumanMove_Up, MainInk_Save),
            Self::weights(Action_Up, SubEffect_Reduction),
            Self::weights(JumpTime_Save, InkRecovery_Up),
            Self::weights(SpecialSpec_Up, SpecialIncrease_Up),
            Self::weights(RespawnSpecialGauge_Save, SubInk_Save),
            Self::weights(OpInkEffect_Reduction, SubSpec_Up),
            Self::weights(SubSpec_Up, MainInk_Save),
            Self::weights(InkRecovery_Up, RespawnSpecialGauge_Save),
            Self::weights(SubSpec_Up, Action_Up),
            Self::weights(SpecialIncrease_Up, Action_Up),
            [2; 14],
            [2; 14],
            [2; 14],
            [2; 14],
        ]
    };

    #[inline]
    #[must_use]
    pub const fn max_num(&self) -> u32 {
        #[allow(clippy::enum_glob_use)]
        use Brand::*;
        match self {
            B97 | B98 | B99 | None => 28,
            _ => 35,
        }
    }

    #[inline]
    const fn weights(unusual: Ability, usual: Ability) -> BrandData {
        let mut data = [2; 14];
        data[unusual as usize] = 1;
        data[usual as usize] = 10;
        data
    }

    #[inline]
    #[must_use]
    pub const fn get_ability(&self, seed: u32) -> Ability {
        let roll = seed % self.max_num();
        self.weighted_ability(roll)
    }

    #[inline]
    #[must_use]
    pub const fn get_ability_drink(&self, seed: u32, drink: Ability) -> Ability {
        let roll = seed % self.max_num_drink(drink);
        self.weighted_ability_drink(roll, drink)
    }

    #[inline]
    #[must_use]
    pub const fn weighted_ability(&self, roll: u32) -> Ability {
        Self::NO_DRINK_RESULT[*self as usize][roll as usize]
    }

    #[inline]
    #[must_use]
    pub const fn weighted_ability_drink(&self, roll: u32, drink: Ability) -> Ability {
        Self::DRINK_RESULT[drink as usize][*self as usize][roll as usize]
    }

    #[inline]
    #[must_use]
    pub const fn max_num_drink(&self, drink: Ability) -> u32 {
        self.max_num() - Self::_WEIGHTS[*self as usize][drink as usize]
    }

    #[inline]
    #[must_use]
    pub const fn from_usize(brand: usize) -> Self {
        #[allow(clippy::enum_glob_use)]
        use Brand::*;
        [
            B00, B01, B02, B03, B04, B05, B06, B07, B08, B09, B10, B11, B15, B16, B17,
            B18, B19, B20, B97, B98, B99, None,
        ][brand]
    }

    #[must_use]
    pub const fn internal_name(&self) -> &'static str {
        [
            "B00", "B01", "B02", "B03", "B04", "B05", "B06", "B07", "B08", "B09",
            "B10", "B11", "B15", "B16", "B17", "B18", "B19", "B20", "B97", "B98",
            "B99", "None",
        ][*self as usize]
    }
}
impl From<&str> for Brand {
    fn from(val: &str) -> Self {
        Self::from_str(val).unwrap()
    }
}
type BrandData = [u32; 14];

pub const fn get_ability(
    seed: &mut u32,
    brand: Brand,
    drink: Option<Ability>,
) -> Ability {
    advance_seed(seed);
    let rv = brand.get_ability(*seed);
    if let Some(drink) = drink {
        if *seed % 100 <= 0x1D {
            drink
        } else {
            advance_seed(seed);
            brand.get_ability_drink(*seed % brand.max_num_drink(drink), drink)
        }
    } else {
        rv
    }
}

pub struct Slot {
    pub ability: Ability,
    pub drink: Option<Ability>,
}

#[cfg(feature = "rayon")]
pub trait CandType = IntoParallelIterator<Item = u32>;
#[cfg(not(feature = "rayon"))]
pub trait CandType = Iterator<Item = u32>;

#[must_use]
pub fn get_initial_seeds<T: CandType>(
    candidates: T,
    max_results: Option<u32>,
    gear_brand: Brand,
    slots: &[Slot],
) -> Vec<u32> {
    #[cfg(feature = "rayon")]
    let results = candidates
        .into_par_iter()
        .filter(|seed| slots_match(*seed, &gear_brand, slots));
    #[cfg(not(feature = "rayon"))]
    let results = candidates.filter(|seed| slots_match(*seed, &gear_brand, slots));
    if let Some(max_results) = max_results {
        #[cfg(feature = "rayon")]
        {
            let count = AtomicU32::new(0);
            let results_vec = Mutex::new(Vec::with_capacity(max_results as usize));
            results.for_each(|result| {
                if count.load(Ordering::Relaxed) < max_results {
                    let mut vec = match results_vec.lock() {
                        Ok(vec) => vec,
                        _ => unreachable!(),
                        // PoisonError
                        // PoisonError can't happen as it happens only
                        // when a thread
                        // with a MutexGuard (obtained here) panics;
                        // this thread can only
                        // panic if obtaining the guard panics, so there
                        // can never be a
                        // PoisonError
                    };
                    vec.push(result);
                    count.fetch_add(1, Ordering::Relaxed);
                }
            });
            match results_vec.into_inner() {
                Ok(vec) => vec,
                _ => unreachable!(), // PoisonError
            }
        }
        #[cfg(not(feature = "rayon"))]
        results.take(max_results as usize).collect()
    } else {
        results.collect()
    }
}

#[must_use]
pub fn get_results<T: CandType>(
    candidates: T,
    max_results: Option<u32>,
    gear_brand: Brand,
    slots: &[Slot],
) -> Vec<u32> {
    let mut results = get_initial_seeds(candidates, max_results, gear_brand, slots);
    for seed in results.iter_mut() {
        for &Slot {
            ability,
            drink,
        } in slots
        {
            assert_eq!(ability, get_ability(seed, gear_brand, drink));
        }
    }
    results
}

#[must_use]
pub fn slots_match(mut seed: u32, &gear_brand: &Brand, slots: &[Slot]) -> bool {
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

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn mine(max_results: u32, gear_brand: u32, slots: &[u32]) -> Vec<u32> {
    let converted = slots
        .iter()
        .map(|&data| {
            let drink = data & 0xFF00;
            let drink = drink >> 8;
            Slot {
                ability: Ability::from_usize((data & 0xFF) as usize),
                drink: if drink == 0xFF {
                    None
                } else {
                    Some(Ability::from_usize(drink as usize))
                },
            }
        })
        .collect::<Vec<_>>();
    get_results(
        0..=u32::MAX,
        Some(max_results),
        Brand::from_usize(gear_brand as usize),
        &converted,
    )
}
