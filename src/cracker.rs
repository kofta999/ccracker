use crate::hasher::md5;
use std::collections::HashMap;

const ASCII_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const MAX_PW_LENGTH: u8 = 4;

fn crack_bruteforce_recur(prefix: String, mut level: u8, goal: &str) -> Option<String> {
    level += 1;
    for c in ASCII_CHARS.chars() {
        let str = String::from(&prefix) + &c.to_string();

        if md5::hash(str.as_bytes()) == goal {
            return Some(str);
        }

        if level < MAX_PW_LENGTH {
            if let Some(result) = crack_bruteforce_recur(str, level, goal) {
                return Some(result);
            }
        }
    }

    None
}

pub fn crack_bruteforce(pw_hash: &str) -> Option<String> {
    crack_bruteforce_recur(String::new(), 0, pw_hash)
}

pub fn crack_dict(dict: String, goal: &str) -> Option<String> {
    for pw in dict.lines() {
        if md5::hash(pw.as_bytes()) == goal {
            return Some(pw.to_string());
        }
    }

    None
}

pub fn crack_rainbow(table: String, goal: &str) -> Option<String> {
    let rainbow_table = gen_map(table);
    if let Some(password) = rainbow_table.get(goal) {
        return Some(password.to_owned());
    }

    None
}

fn gen_map(table: String) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();

    table.lines().for_each(|s| {
        let mut split = s.split('\t');
        if let (Some(hash), Some(password)) = (split.next(), split.next()) {
            hashmap.insert(hash.to_string(), password.to_string());
        }
    });

    hashmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Heavy computations"]
    fn test_4_letter_password() {
        let pw1 = "7a95bf926a0333f57705aeac07a362a2";
        let pw2 = "08054846bbc9933fd0395f8be516a9f9";

        assert_eq!(crack_bruteforce(pw1), Some("PASS".to_string()));
        assert_eq!(crack_bruteforce(pw2), Some("CODE".to_string()));
    }

    #[test]
    fn test_password_in_dict() {
        let pw = "040173afc2e9520e65a1773779691d3e";

        assert_eq!(
            crack_dict(String::from("passw0rd!"), pw),
            Some("passw0rd!".to_string())
        )
    }

    #[test]
    fn test_gen_table() {
        let s = "
e10adc3949ba59abbe56e057f20f883e	123456
25f9e794323b453885f5181f1b624d0b	123456789
d8578edf8458ce06fbc5bb76a58c5ca4	qwerty
5f4dcc3b5aa765d61d8327deb882cf99	password
";

        let map = gen_map(s.to_string());
        let expected_map = HashMap::from([
            (
                "d8578edf8458ce06fbc5bb76a58c5ca4".to_string(),
                "qwerty".to_string(),
            ),
            (
                "e10adc3949ba59abbe56e057f20f883e".to_string(),
                "123456".to_string(),
            ),
            (
                "5f4dcc3b5aa765d61d8327deb882cf99".to_string(),
                "password".to_string(),
            ),
            (
                "25f9e794323b453885f5181f1b624d0b".to_string(),
                "123456789".to_string(),
            ),
        ]);

        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_rainbow_password_exists() {
        let s = "
e10adc3949ba59abbe56e057f20f883e	123456
25f9e794323b453885f5181f1b624d0b	123456789
d8578edf8458ce06fbc5bb76a58c5ca4	qwerty
5f4dcc3b5aa765d61d8327deb882cf99	password
";

        assert_eq!(
            crack_rainbow(s.to_string(), "5f4dcc3b5aa765d61d8327deb882cf99"),
            Some("password".to_string())
        );
    }

    #[test]
    fn test_rainbow_password_not_exists() {
        let s = "
e10adc3949ba59abbe56e057f20f883e	123456
25f9e794323b453885f5181f1b624d0b	123456789
d8578edf8458ce06fbc5bb76a58c5ca4	qwerty
5f4dcc3b5aa765d61d8327deb882cf99	password
";

        assert_eq!(
            crack_rainbow(s.to_string(), "5f4dcc3b5aa765d61d8327deb882cf98"),
            None
        );
    }
}
