use crate::md5::hash_md5;
const ASCII_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const MAX_PW_LENGTH: u8 = 4;

fn crack_bruteforce_recur(prefix: String, mut level: u8, goal: &str) -> Option<String> {
    level += 1;
    for c in ASCII_CHARS.chars() {
        let str = String::from(&prefix) + &c.to_string();

        if hash_md5(&str) == goal {
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
        if hash_md5(pw) == goal {
            return Some(pw.to_string());
        }
    }

    None
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
}
