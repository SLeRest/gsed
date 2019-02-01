use std::path::Path;
use gsed::replace::Replace;
use gsed::opt::Opt;
use structopt::StructOpt;

fn main() {
    let mut opt = Opt::from_args();
    if opt.paths.is_empty() {
        opt.paths.push("./".to_string());
    }
    let replace: Replace = Replace::new(opt);

    
    for path in replace.opt.paths.iter() {
        let p: &Path = Path::new(path.as_str());

        if p.exists() {
            if p.is_file() {
                replace.replace_file(&path);
            } else if p.is_dir() {
               replace.replace_dir(p);
            } else {
                eprintln!("Error: gsed: wrong type of file: {}", path);
            }
        } else {
            eprintln!("Error: gsed: no such file or directory: {}", path);
        }
    }
}
