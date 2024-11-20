fn md5_hash(message: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_md5() {
        let message = "test";
        let hash = "098f6bcd4621d373cade4e832627b4f6";

        assert_eq!(hash, md5_hash(message))
    }
}
