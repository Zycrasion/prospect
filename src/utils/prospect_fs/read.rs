use std::{fs, path::Path};

pub fn read_file<S : AsRef<str>>(path : S, default : S) -> String
{
    let file = fs::read_to_string(Path::new(path.as_ref()));
    let file = file.unwrap_or(default.as_ref().to_owned());
    file
}

pub fn read_file_option<S : AsRef<str>>(path : S) -> Option<String>
{
    let file = fs::read_to_string(Path::new(path.as_ref()));
    let file = if let Ok(cts) = file {Some(cts)} else {None};
    file
}

pub fn read_file_panic<S : AsRef<str>>(path : S) -> String
{
    let file = fs::read_to_string(Path::new(path.as_ref()));
    let file = file.unwrap();
    file
}

pub fn read_file_with_respect_to_cwd<S : AsRef<str>>(path : S) -> String
{
    let path = std::env::current_dir().unwrap().join(Path::new(path.as_ref()));
    fs::read_to_string(path).unwrap()
}

pub fn read_file_with_respect_to_cwd_bytes<S : AsRef<str>>(path : S) -> Vec<u8>
{
    let path = std::env::current_dir().unwrap().join(Path::new(path.as_ref()));
    fs::read(path).unwrap()
}