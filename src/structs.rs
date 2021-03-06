use num_enum::TryFromPrimitive;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub(crate) struct Cli {
    pub(crate) day: u8,
    pub(crate) step: u8,
    pub(crate) test: Option<String>, // #[structopt(parse(from_os_str))]
                                     // input: std::path::PathBuf
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum Day {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub(crate) enum Step {
    One = 1,
    Two = 2,
}
