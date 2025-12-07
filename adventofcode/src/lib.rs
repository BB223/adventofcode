use std::{env, fmt::Display, fs, path::PathBuf};

use reqwest::{
    blocking::Client,
    header::{COOKIE, HeaderMap, HeaderValue},
};

#[macro_export]
macro_rules! advent_of_code {
    ($day:expr, $exp1:expr, $exp2:expr) => {
        fn main() {
            let input = $crate::read_input($day);
            $crate::print_result(&part_one(&input), "Part 1");
            $crate::print_result(&part_two(&input), "Part 2");
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part_one() {
                let result = part_one(&$crate::read_example($day));
                assert_eq!(result, $exp1);
            }

            #[test]
            fn test_part_two() {
                let result = part_two(&$crate::read_example($day));
                assert_eq!(result, $exp2);
            }
        }
    };
}

pub fn print_result<T: Display>(result: &Option<T>, part: &str) {
    match result {
        Some(result) => {
            let str = format!("{part}: {result}");
            println!("{str}");
        }
        None => {
            println!("{part}: ✖             ");
        }
    }
}

#[derive(Copy, Clone)]
enum DataFolder {
    Inputs,
    Examples,
}

impl DataFolder {
    fn folder(&self) -> &'static str {
        match self {
            DataFolder::Inputs => "inputs",
            DataFolder::Examples => "examples",
        }
    }
}

pub fn read_input(day: u8) -> String {
    read_file_or_download(DataFolder::Inputs, day)
}

pub fn read_example(day: u8) -> String {
    read_file_or_download(DataFolder::Examples, day)
}

fn read_file_or_download(folder: DataFolder, day: u8) -> String {
    let path = build_path(folder, day);

    if path.exists() {
        return fs::read_to_string(path).expect("Failed to read file");
    }

    match folder {
        DataFolder::Inputs => {
            let content = get_input(day);
            write_file(&path, &content);
            content
        }
        DataFolder::Examples => panic!(
            "Example file missing: {}. No auto-fetch for examples.",
            path.display()
        ),
    }
}

fn build_path(folder: DataFolder, day: u8) -> PathBuf {
    let root = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(root)
        .join("data")
        .join(folder.folder())
        .join(format!("{:02}.txt", day))
}

fn get_input(day: u8) -> String {
    let client = build_client();

    client
        .get(format!("https://adventofcode.com/2025/day/{}/input", day))
        .send()
        .expect("Request failed")
        .text()
        .expect("Body failed")
}

fn build_client() -> Client {
    let mut headers = HeaderMap::new();

    let cookie = read_cookie();
    let cookie_header_value = format!("session={}", cookie);
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&cookie_header_value).expect("Invalid cookie header"),
    );

    let ua_string = build_user_agent();

    Client::builder()
        .user_agent(ua_string)
        .default_headers(headers)
        .build()
        .expect("Failed to build reqwest client")
}

fn read_cookie() -> String {
    fs::read_to_string(cookie_path())
        .expect("Failed to read session cookie file")
        .trim()
        .to_string()
}

fn cookie_path() -> PathBuf {
    let home = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".config")
        });

    home.join("adventofcode").join("session")
}

fn build_user_agent() -> String {
    let repo = env!("CARGO_PKG_REPOSITORY");
    let authors = env!("CARGO_PKG_AUTHORS");

    format!("{repo} by {authors}")
}

fn write_file(path: &PathBuf, data: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directory");
    }

    fs::write(path, data).expect("Failed to write file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let result = read_input(0);
        assert_eq!(result, "test\n");
    }

    #[test]
    fn test_read_example() {
        let result = read_example(0);
        assert_eq!(result, "test\n");
    }
}
