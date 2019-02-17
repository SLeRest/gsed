// extern crate regex;

use tempfile::NamedTempFile;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs;
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
        let mut i = 1;
        for line in buf.lines() {
            let mut l = line.unwrap();
            if l.contains(self.search.as_str()) {
                if self.opt.all {
                    l = l.replace(self.search.as_str(),
                                    self.replace.as_str()).to_string();
                } else {
                    print!("l {}: {}\ty/n: ", i, l);
                    ::std::io::stdout().flush();
                    let mut input = String::new();
                    match ::std::io::stdin().read_line(&mut input) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error: gsed: {}", e);
                        },
                    }
                    if input.trim() == "y" {
                        l = l.replace(self.search.as_str(),
                                    self.replace.as_str()).to_string();
                    }
                }
            }
            l.push('\n');
            tmpfile.write_all(&l.as_bytes()).unwrap();
            i += 1;
        }
    }

    fn loop_replace_file_regex (&self, buf: BufReader<&File>,
                                tmpfile: &mut NamedTempFile)
    {
        let regex = Regex::new(self.search.as_str()).unwrap();
        let mut i = 1;
        for line in buf.lines() {
            let mut l = line.unwrap();
            if regex.is_match(l.as_str()) {
                if self.opt.all {
                    l = regex.replace_all(&l,
                                self.replace.as_str()).to_string();
                } else {
                    print!("l {}: {}\ty/n: ", i, l);
                    ::std::io::stdout().flush();
                    let mut input = String::new();
                    match ::std::io::stdin().read_line(&mut input) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error: gsed: {}", e);
                        },
                    }
                    if input.trim() == "y" {
                        l = regex.replace_all(&l,
                                    self.replace.as_str()).to_string();
                    }
                }
            }
            l.push('\n');
            tmpfile.write_all(&l.as_bytes()).unwrap();
            i += 1;
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

    pub fn replace_dir(&self, path: &Path) {
        let readdir = match fs::read_dir(path) {
            Ok(rdir) => rdir,
            Err(e) => {
                eprintln!("Error: gsed: {}: {}", e, path.to_str().unwrap());
                return ;
            },
        };
        let mut vdirs: Vec<PathBuf> = Vec::new();

        for file in readdir {
            let f = match file {
                Ok(ff) => ff,
                Err(e) => {
                    eprintln!("Error: gsed: {}", e);
                    continue ;
                }
            };
            let ftype = match f.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    eprintln!("Error: gsed: {}", e);
                    return ;
                },
            };
            if ftype.is_file() {
                self.replace_file(
                    &f.path()
                    .into_os_string()
                    .into_string()
                    .unwrap()
                );
            } else if ftype.is_dir() {
                if self.opt.recursive {
                    vdirs.push(f.path());
                } else {
                    continue ;
                }
            } else {
                eprintln!("Error: gsed: wrong type of file");
            }
        }
        for vdir in vdirs {
            self.replace_dir(vdir.as_path());
        }
    }
}
