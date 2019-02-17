use std::path::PathBuf;
use crate::opt::Opt;
use std::fmt;

#[derive(Debug)]
pub struct Info {
    pub pattern: String,
    pub replace: String,
    pub file: Vec<PathBuf>,
    pub dir: Vec<PathBuf>,
    pub recursive: bool,
    pub regex: bool,
    pub all: bool,
}

impl fmt::Display for Info {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print!("{}", self.pattern);
        print!("{}", self.replace);
        print!("{:?}", self.file);
        print!("{:?}", self.dir);
        print!("{}", self.recursive);
        print!("{}", self.regex);
        print!("{}", self.all);
        Ok(())
    }
}

impl Info {
    pub fn new(opt: Opt, file: Vec<PathBuf>, dir: Vec<PathBuf>) -> Info {
        Info {
            pattern: opt.pattern.unwrap(),
            replace: opt.replace.unwrap(),
            file: file,
            dir: dir,
            recursive: opt.recursive,
            regex: opt.regex,
            all: opt.all
        }
    }
}
