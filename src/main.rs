extern crate regex;

use structopt::{clap, StructOpt};
use tempfile::NamedTempFile;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;


#[derive(Debug, StructOpt)]
#[structopt(name =  "gsed")]
#[structopt(raw(long_version = "option_env!(\"LONG_VERSION\").unwrap_or(env!(\"CARGO_PKG_VERSION\"))"))]
#[structopt(raw(setting = "clap::AppSettings::ColoredHelp"))]
#[structopt(raw(setting = "clap::AppSettings::DeriveDisplayOrder"))]


struct Opt {

    /// Keyword for search
    #[structopt(name = "SEARCH", required_unless = "file")]
    search: Option<String>,

    #[structopt(name = "REPLACE", required_unless = "rep")]
    replace: Option<String>,

    /// Search paths
    #[structopt(name = "path")]
    paths: Vec<String>,

    /// Opt for recursive
    #[structopt(long = "recursive", short = "R")]
    directory: bool,

    /// Opt for regex match
    #[structopt(long = "regex", short = "r")]
    regex: bool,

}

/// lire le Path
///     si c'est un regular file
///     si c'est un dir on parse tout les file
///     si c'est un dir et directoy == true
///         on ajoute les dir qu'on croise quand on parse a opt.path
///    println!("{:?}", opt);
///
fn replace_regex(path: &String, search: &str, replace: &str) {
    let regex = Regex::new(search).unwrap();
    let file = File::open(path).unwrap();
    let buf = BufReader::new(&file);
    let mut tmpfile = NamedTempFile::new().unwrap();

    for line in buf.lines() {
        let mut l = line.unwrap();
        l = regex.replace_all(&l, replace).to_string();
        l.push('\n');
        tmpfile.write_all(&l.as_bytes()).unwrap();
    }

    let metadata = file.metadata().unwrap();
    ::std::fs::set_permissions(tmpfile.path(), metadata.permissions()).unwrap();
    tmpfile.persist(path).unwrap();
}

fn replace_regular(path: &String, search: &str, replace: &str) {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(&file);
    let mut tmpfile = NamedTempFile::new().unwrap();

    for line in buf.lines() {
        let mut l = line.unwrap();
        l = l.replace(search, replace);
        l.push('\n');
        tmpfile.write_all(&l.as_bytes()).unwrap();
    }
    let metadata = file.metadata().unwrap();
    ::std::fs::set_permissions(tmpfile.path(), metadata.permissions()).unwrap();
    tmpfile.persist(path).unwrap();
}


fn main() {
    let mut opt = Opt::from_args();

    if opt.paths.is_empty() {
        opt.paths.push("./".to_string());
    }

    let search = opt.search.as_ref().unwrap();
    let replace = opt.replace.as_ref().unwrap();

    for path in opt.paths.iter() {
        let p: &Path = Path::new(path.as_str());

        if p.exists() {
            if p.is_file() && !opt.regex {
                replace_regular(&path, search, replace);
            } else if p.is_file() && opt.regex {
                replace_regex(&path, search, replace);
            } else {
                eprintln!("Error: gsed: wrong type of file: {}", path);
            }
        } else {
            eprintln!("Error: gsed: no such file or directory: {}", path);
        }
    }
}
