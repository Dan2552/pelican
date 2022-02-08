pub fn is_main() -> bool {
    std::thread::current().name() == Some("main")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_best_we_can() {
        std::thread::spawn(move || {
            assert_eq!(is_main(), false);
        });
    }
}
