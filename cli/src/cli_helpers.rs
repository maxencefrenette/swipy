use clap::ArgMatches;
use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_arg<T>(matches: &ArgMatches, name: &str) -> T
where
    T: FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    matches.value_of(name).unwrap().parse::<T>().unwrap()
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Human,
    Json,
}

impl FromStr for OutputFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "human" => Ok(OutputFormat::Human),
            "json" => Ok(OutputFormat::Json),
            _ => Err(()),
        }
    }
}

impl OutputFormat {
    pub fn possible_values() -> &'static [&'static str] {
        &["human", "json"]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VFunctionChoice {
    Legacy,
    NTupleSmall,
}

impl FromStr for VFunctionChoice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "legacy" => Ok(VFunctionChoice::Legacy),
            "n_tuple_small" => Ok(VFunctionChoice::NTupleSmall),
            _ => Err(()),
        }
    }
}

impl VFunctionChoice {
    pub fn possible_values() -> &'static [&'static str] {
        &["legacy", "n_tuple_small"]
    }
}
