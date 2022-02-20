pub struct Bundle {}

impl Bundle {
    pub fn path_for_resource<'a>(file: &str) -> String {
        if file.starts_with("/") {
            String::from(file)
        } else {
            match std::env::current_exe() {
                Ok(mut path) => {
                    path.pop();
                    path.push("resources");
                    path.push(file);
                    String::from(path.to_str().unwrap())
                }
                Err(_) => format!("./resources/{}", file)
            }
        }
    }
}
