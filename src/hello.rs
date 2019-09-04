pub fn hello(s: &str) -> String {
    format!("Hello, {}!", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello("World"), "Hello, World!");
    }
}
