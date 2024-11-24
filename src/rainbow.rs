use crate::hasher::md5;
use std::{fs::File, io::Write};

const ASCII_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn create_n_len_table(n: u8) {
    let mut file = File::create(format!("rainbow_{n}.txt")).unwrap();

    bruteforce(n, String::new(), 0, &mut |s| {
        writeln!(file, "{}\t{}", md5::hash(s.as_bytes()), s).unwrap();
    });
}

pub fn create_dict_table(dict: String) {
    let mut file = File::create("rainbow_dict.txt").unwrap();
    for pw in dict.lines() {
        writeln!(file, "{}\t{}", md5::hash(pw.as_bytes()), pw).unwrap();
    }
}

fn bruteforce(n: u8, prefix: String, mut level: u8, cb: &mut dyn FnMut(&str)) {
    level += 1;
    for c in ASCII_CHARS.chars() {
        let str = String::from(&prefix) + &c.to_string();
        cb(&str);

        if level < n {
            bruteforce(n, str, level, cb);
        }
    }
}
