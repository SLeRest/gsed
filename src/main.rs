use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};
use gsed::matche::Matche;
use gsed::info::Info;
use gsed::opt::Opt;
use structopt::StructOpt;

fn gsed_file(info: &Info) {
    let mut matches: Vec<Matche> = Vec::new();
    let files = info.file.iter();

    for file in files {
        let path = match file.to_str() {
            Some(v) => v.to_string(),
            None => { continue; },
        };
        let mut f = match File::open(path.as_str()) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: gsed: {}", e);
                continue ;
            }
        };
        let mut src = String::new();
        match f.read_to_string(&mut src) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: gsed: {}", e);
                continue ;
            },
        };
        let mut new_match = Matche::search_file(src.as_bytes(), info.pattern.as_bytes(), &path);
        matches.append(&mut new_match);
    }
    println!("{:#?}", matches);
}

fn main() {
    let mut opt: Opt = Opt::from_args();
    let matches: Vec<Matche> = Vec::new();
    if opt.paths.is_empty() {
        opt.paths.push("./".to_string());
    }

    let mut path_file: Vec<PathBuf> = Vec::new();
    let mut path_dir: Vec<PathBuf> = Vec::new();

    for path in opt.paths.iter() {
        let p: &Path = Path::new(path.as_str());
        if p.exists() {
            if p.is_file() {
                path_file.push(p.to_path_buf());
            } else if p.is_dir() {
                path_dir.push(p.to_path_buf());
            } else {
                eprintln!("Error: gsed: wrong type of file: {}", path);
            }
        } else {
            eprintln!("Error: gsed: no such file or directory: {}", path);
        }
    }

    let info = Info::new(opt, path_file, path_dir);

    gsed_file(&info);
}
