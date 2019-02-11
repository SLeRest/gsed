use structopt::{StructOpt, clap};

#[derive(Debug, StructOpt)]
#[structopt(name =  "gsed")]
#[structopt(raw(long_version = "option_env!(\"LONG_VERSION\").unwrap_or(env!(\"CARGO_PKG_VERSION\"))"))]
#[structopt(raw(setting = "clap::AppSettings::ColoredHelp"))]
#[structopt(raw(setting = "clap::AppSettings::DeriveDisplayOrder"))]

pub struct Opt {

    /// Search
    #[structopt(name = "SEARCH", required_unless = "file")]
    pub search: Option<String>,

    /// Replace
    #[structopt(name = "REPLACE", required_unless = "rep")]
    pub replace: Option<String>,

    /// Paths
    #[structopt(name = "path")]
    pub paths: Vec<String>,

    /// Recursive
    #[structopt(long = "recursive", short = "R")]
    pub recursive: bool,

    /// Regex
    #[structopt(long = "regex", short = "r")]
    pub regex: bool,

    /// All
    #[structopt(long = "all", short = "a")]
    pub all: bool,
}
