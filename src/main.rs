use gsed::gsed::Gsed;
use gsed::opt::Opt;

use structopt::StructOpt;

fn main() {
    let mut opt = Opt::from_args();
    if opt.paths.is_empty() {
        opt.paths.push("./".to_string());
    }
    let gsed: Gsed = Gsed::new(opt);

    println!("{:?}", gsed.opt);
    /*
    for path in opt.paths.iter() {
        let p: &Path = Path::new(path.as_str());

        if p.exists() {
            if p.is_file() {
                replace_file(&path, search, replace, &opt.regex);
//            } else if p.is_dir() && !opt.recursive && !opt.regex {
 //               replace_dir_regex(&path, search, replace);
            } else {
                eprintln!("Error: gsed: wrong type of file: {}", path);
            }
        } else {
            eprintln!("Error: gsed: no such file or directory: {}", path);
        }
    }
    */
}
