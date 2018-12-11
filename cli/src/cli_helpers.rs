use clap::ArgMatches;
use std::fmt::Debug;
use std::str::FromStr;
use strum_macros::EnumString;

pub fn parse_arg<T>(matches: &ArgMatches, name: &str) -> T
where
    T: FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    matches.value_of(name).unwrap().parse::<T>().unwrap()
}

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum OutputFormat {
    Human,
    Json,
}

impl OutputFormat {
    pub fn possible_values() -> &'static [&'static str] {
        &["human", "json"]
    }
}

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum VFunctionChoice {
    Legacy,
    NTupleSmall,
    NTupleMedium,
}

impl VFunctionChoice {
    pub fn possible_values() -> &'static [&'static str] {
        &["legacy", "n_tuple_small", "n_tuple_medium"]
    }
}
