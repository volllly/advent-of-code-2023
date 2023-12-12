use std::{
    collections::HashMap,
    ops::{Deref, Range},
    str::FromStr,
};

use chumsky::{prelude::*, text::newline};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use rangemap::RangeMap;
use rayon::prelude::*;
use tap::Pipe;

advent_of_code::solution!(5);

macro_rules! make_key_types {
    ($($name:ident),*) => {
        $(#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)] struct $name(u64);)*

        $(
            impl rangemap::StepLite for $name {
                fn add_one(&self) -> Self {
                    $name(self.0 + 1)
                }
                fn sub_one(&self) -> Self {
                    $name(self.0 - 1)
                }
            }

            impl From<u64> for $name {
                fn from(value: u64) -> Self {
                    Self(value)
                }
            }

            impl std::ops::Deref for $name {
                type Target = u64;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        )*
    };
}
make_key_types!(
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
);

#[derive(Debug)]
struct Mapping {
    seeds: Vec<Seed>,
    soil: RangeMap<Seed, Soil>,
    fertilizer: RangeMap<Soil, Fertilizer>,
    water: RangeMap<Fertilizer, Water>,
    light: RangeMap<Water, Light>,
    temperature: RangeMap<Light, Temperature>,
    humidity: RangeMap<Temperature, Humidity>,
    location: RangeMap<Humidity, Location>,
}

impl Mapping {
    fn map<F, T>(&self, from: F, to: Option<(&Range<F>, &T)>) -> T
    where
        F: Deref<Target = u64>,
        F: Clone,
        F: Ord,
        T: Deref<Target = u64>,
        T: From<u64>,
    {
        if let Some((k, v)) = to {
            T::from(**v + *from - *k.start())
        } else {
            T::from(*from)
        }
    }

    pub fn soil(&self, seed: Seed) -> Soil {
        self.map(seed, self.soil.get_key_value(&seed))
    }

    pub fn fertilizer(&self, soil: Soil) -> Fertilizer {
        self.map(soil, self.fertilizer.get_key_value(&soil))
    }

    pub fn water(&self, fertilizer: Fertilizer) -> Water {
        self.map(fertilizer, self.water.get_key_value(&fertilizer))
    }

    pub fn light(&self, water: Water) -> Light {
        self.map(water, self.light.get_key_value(&water))
    }

    pub fn temperature(&self, light: Light) -> Temperature {
        self.map(light, self.temperature.get_key_value(&light))
    }

    pub fn humidity(&self, temperature: Temperature) -> Humidity {
        self.map(temperature, self.humidity.get_key_value(&temperature))
    }

    pub fn location(&self, humidity: Humidity) -> Location {
        self.map(humidity, self.location.get_key_value(&humidity))
    }

    pub fn location_from_seed(&self, seed: Seed) -> Location {
        self.soil(seed)
            .pipe(|soil| self.fertilizer(soil))
            .pipe(|fertilizer| self.water(fertilizer))
            .pipe(|water| self.light(water))
            .pipe(|light| self.temperature(light))
            .pipe(|temperature| self.humidity(temperature))
            .pipe(|humidity| self.location(humidity))
    }

    pub fn locations(&self) -> Vec<Location> {
        self.seeds
            .iter()
            .copied()
            .map(|seed| self.location_from_seed(seed))
            .collect::<Vec<_>>()
    }

    pub fn min_locations(&self) -> Option<Location> {
        let progress = MultiProgress::new();
        let template = ProgressStyle::with_template(
            "{msg} [{elapsed_precise}] {bar:50.cyan/blue} {pos}/{len} {eta}",
        )
        .unwrap();
        let outer = progress.add(
            ProgressBar::new(self.seeds.len() as u64 / 2)
                .with_message("Ranges")
                .with_style(template.clone()),
        );
        outer.tick();

        self.seeds
            .iter()
            .tuples()
            .map(|(s, e)| (**s..(**s + **e)))
            .par_bridge()
            .fold(
                || Location(u64::MAX),
                |mut acc, seeds| {
                    let inner = progress.add(
                        ProgressBar::new(seeds.end() - seeds.start())
                            .with_message("Seeds ")
                            .with_style(template.clone())
                            .with_position(0),
                    );
                    for seed in seeds {
                        acc = acc.min(self.location_from_seed(Seed(seed)));
                        inner.inc(1);
                    }
                    inner.finish_with_message("Done  ");
                    outer.inc(1);
                    acc
                },
            )
            .min()
    }
}

impl FromStr for Mapping {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds = just("seeds:").padded().ignore_then(
            text::int(10)
                .map(|s: String| Seed(s.parse::<u64>().unwrap()))
                .separated_by(just(' '))
                .collect::<Vec<_>>(),
        );

        let ranges = text::int(10)
            .map(|s: String| s.parse::<u64>().unwrap())
            .separated_by(just(' '))
            .exactly(3);

        let mapping = text::ident()
            .then(just("-to-"))
            .ignore_then(text::ident())
            .then_ignore(just(" map:").then(newline()))
            .then(ranges.separated_by(newline()));

        seeds
            .then_ignore(newline().repeated())
            .then(
                mapping
                    .then_ignore(newline().repeated())
                    .repeated()
                    .exactly(7)
                    .collect::<HashMap<_, _>>(),
            )
            .map(|(seeds, mappings)| {
                macro_rules! get_mappings {
                    ($name:expr, $key:ident, $value:ident) => {
                        mappings[$name]
                            .iter()
                            .map(|range| {
                                ($key(range[1])..$key(range[1] + range[2]), $value(range[0]))
                            })
                            .collect::<RangeMap<$key, $value>>()
                    };
                }
                Mapping {
                    seeds,
                    soil: get_mappings!("soil", Seed, Soil),
                    fertilizer: get_mappings!("fertilizer", Soil, Fertilizer),
                    water: get_mappings!("water", Fertilizer, Water),
                    light: get_mappings!("light", Water, Light),
                    temperature: get_mappings!("temperature", Light, Temperature),
                    humidity: get_mappings!("humidity", Temperature, Humidity),
                    location: get_mappings!("location", Humidity, Location),
                }
            })
            .parse(s)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Mapping::from_str(input)
        .unwrap()
        .locations()
        .into_iter()
        .min()
        .map(|l| *l)
}

pub fn part_two(input: &str) -> Option<u64> {
    Mapping::from_str(input)
        .unwrap()
        .min_locations()
        .map(|l| *l)
}

#[cfg(test)]
mod tests {
    use tailsome::IntoOption;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 35.into_some());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 46.into_some());
    }
}
