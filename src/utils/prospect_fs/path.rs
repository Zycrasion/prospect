use std::path::{Path, PathBuf};

pub fn path_with_respect_to_cwd_str<S : AsRef<str>>(path : S) -> String
{
    std::env::current_dir().unwrap().join(Path::new(path.as_ref())).to_str().unwrap().to_string()
}

pub fn path_with_respect_to_cwd<S : AsRef<str>>(path : S) -> PathBuf
{
    std::env::current_dir().unwrap().join(Path::new(path.as_ref()))
}