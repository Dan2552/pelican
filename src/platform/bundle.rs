use std::env;

pub struct Bundle {}

impl Bundle {
    pub fn path_for_resource<'a>(file: &str) -> String {
        if file.starts_with("/") {
            String::from(file)
        } else {
            let mut current_exe = env::current_exe().unwrap();
            current_exe.pop();
            current_exe.push("resources");
            current_exe.push(file);
            String::from(current_exe.to_str().unwrap())
        }
    }
}
