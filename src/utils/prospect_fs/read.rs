use std::{fs, path::Path};

pub fn read_file<S : AsRef<str>>(path : S, default : S) -> String
{
    let file = fs::read_to_string(Path::new(path.as_ref()));
    let file = file.unwrap_or(default.as_ref().to_owned());
    file
}

pub fn read_file_panic<S : AsRef<str>>(path : S) -> String
{
    let file = fs::read_to_string(Path::new(path.as_ref()));
    let file = file.unwrap();
    file
}