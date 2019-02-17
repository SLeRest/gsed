use std::fmt;

#[derive(Debug)]
pub struct Matche {
    pub path: String,
    pub n_line: u64,
    pub str_line: String,
    pub index: usize,
    pub replace: bool,
}

impl fmt::Display for Matche {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print!("{}", self.path);
        print!("{}", self.n_line);
        print!("{:?}", self.str_line);
        print!("{:?}", self.index);
        print!("{}", self.replace);
        Ok(())
    }
}

impl Matche {

    pub fn get_strline(i: &mut usize, src: &[u8]) -> String
    {
        let mut line = String::new();
        let mut j: usize = if *i == 0  && src[0] as char != '\n' { *i } else { *i + 1 };

        *i = if *i == 0 && src[0] as char != '\n' { *i } else { *i + 1 };
        while src[j] as char != '\0' && src[j] as char != '\n' {
            line.push(src[j] as char);
            j += 1;
        }
        line
    }

    pub fn search_file(src: &[u8], pat: &[u8], path: &String) -> Vec<Matche> {
        let src_len = src.len();
        let pat_len = pat.len();
        let mut matches = Vec::new();
        let mut i: usize = 0;
        let mut nline: u64 = 0;
        let mut line = String::new();

        while i < src_len - pat_len + 1 {
            if src[i] as char == '\n' || i == 0 {
                line = Matche::get_strline(&mut i, src);
                nline += 1;
            }
            if src[i] == pat[0] {
                let mut bmatch = true;
                let mut j = 1;
                while j < pat_len {
                    if pat[j] != src[i + j] {
                        bmatch = false;
                        break ;
                    }
                    j += 1;
                }
                if bmatch {
                    matches.push(
                        Matche {
                            path: path.clone(),
                            n_line: nline,
                            str_line: line.clone(),
                            index: i,
                            replace: false,
                        }
                    );
                }
            }
            i += 1;
        }
        matches
    }
}
