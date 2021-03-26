pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod test {
    #[test]
    fn test_version() {
        assert_eq!(super::VERSION, "0.1.0"); 
    }
}
