use structopt::{clap, StructOpt};
use tempfile::NamedTempFile;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

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

    #[structopt(long = "directory", short = "d")]
    directory: bool,

}

/// lire le Path
///     si c'est un regular file
///     si c'est un dir on parse tout les file
///     si c'est un dir et directoy == true
///         on ajoute les dir qu'on croise quand on parse a opt.path
///    println!("{:?}", opt);

fn gsed_regular_file(path: &String, search: &str, replace: &str) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: gsed: {}: {}", e, path);
            return ;
        },
    };
    /// pour avoir les permitions
    /// TODO si opt.interactive est false remplacer sans chercher a comprendre
    let buf = BufReader::new(&file);
    let mut tmpfile = match NamedTempFile::new() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: gsed: {}", e);
            drop(file);
            ::std::process::exit(1);
        },
    };

    for line in buf.lines() {
        let mut l = line.unwrap();
        if l.contains(search) == true {
            l = l.replace(search, replace);
        }
        match tmpfile.write_all(&l.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error: gsed: {}", e);
                drop(file);
                ::std::process::exit(1);
            },
        };
    }
    let metadata = match file.metadata() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: gsed: {}", e);
            drop(file);
            ::std::process::exit(1);
        },
    };
    ::std::fs::set_permissions(tmpfile.path(), metadata.permissions());
    tmpfile.persist(path);
}


fn main() {
    let mut opt = Opt::from_args();

    if opt.paths.is_empty() {
        opt.paths.push("./".to_string());
    }

    let search = opt.search.as_ref().unwrap();
    let replace = opt.replace.as_ref().unwrap();
    for path in opt.paths.iter() {
    //    let p: &Path = Path::new(path.as_str());

        gsed_regular_file(&path, &search, &replace);
        /*
           if p.exist() == true {
           if p.is_file() == true {
           gsed_regular_file(&path, &opt);
           } else if p.is_dir() == true {
           gsed_directory_recursive(&p, &opt);
           } else {
           eprintn!("Error: gsed: wrong type of file: {}", path);
           ::std::process::exit(1);
           }
           }
           */
    }
}
