// extern crate regex;

use tempfile::NamedTempFile;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use crate::opt::Opt;



pub struct Replace {
    pub opt: Opt,
    pub search: String,
    pub replace: String,
    // matchs: Vec<Match>,
}

impl Replace {
    pub fn new(opt: Opt) -> Replace {
        let search = opt.search.as_ref().unwrap().to_string();
        let replace = opt.replace.as_ref().unwrap().to_string();

        Replace { opt: opt , search: search, replace: replace }
    }

    fn loop_replace_file_regular (&self, buf: BufReader<&File>,
                                  tmpfile: &mut NamedTempFile)
    {
        for line in buf.lines() {
            let mut l = line.unwrap();
            l = l.replace(self.search.as_str(), self.replace.as_str()).to_string();
            l.push('\n');
            tmpfile.write_all(&l.as_bytes()).unwrap();
        }
    }

    fn loop_replace_file_regex (&self, buf: BufReader<&File>,
                                tmpfile: &mut NamedTempFile)
    {
        let regex = Regex::new(self.search.as_str()).unwrap();
        for line in buf.lines() {
            let mut l = line.unwrap();
            l = regex.replace_all(&l, self.replace.as_str()).to_string();
            l.push('\n');
            tmpfile.write_all(&l.as_bytes()).unwrap();
        }
    }

    pub fn replace_file(&self, path: &String) {
        let file = File::open(path).unwrap();
        let buf = BufReader::new(&file);
        let mut tmpfile = NamedTempFile::new().unwrap();

        if self.opt.regex {
            self.loop_replace_file_regex(buf, &mut tmpfile)
        } else {
            self.loop_replace_file_regular(buf, &mut tmpfile)
        }

        let metadata = file.metadata().unwrap();
        ::std::fs::set_permissions(tmpfile.path(), metadata.permissions()).unwrap();
        tmpfile.persist(path).unwrap();
    }
}
