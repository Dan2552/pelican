pub struct Bundle {}

impl Bundle {
    pub fn path_for_resource<'a>(file: &str) -> String {
        if file.starts_with("/") {
            String::from(file)
        } else {
            match std::env::current_exe() {
                Ok(mut path) => {
                    path.pop();

                    if path.to_str().unwrap().ends_with("target/debug") {
                        path.pop();
                        path.pop();
                    }

                    path.push("resources");
                    path.push(file);
                    String::from(path.to_str().unwrap())
                }
                Err(_) => format!("./resources/{}", file)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundle_path_for_resource() {
        let path = Bundle::path_for_resource("/foo/bar/baz.txt");
        assert_eq!(path, "/foo/bar/baz.txt");

        let path = Bundle::path_for_resource("foo/bar/baz.txt");
        assert_ne!(path, "foo/bar/baz.txt");
        assert_ne!(path, "resources/foo/bar/baz.txt");
        assert!(path.ends_with("resources/foo/bar/baz.txt"))
    }
}
