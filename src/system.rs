use std::path::PathBuf;

pub trait SystemShim {
    fn root_path(&self) -> PathBuf;
}

pub struct Ev3DevSystem;

impl SystemShim for Ev3DevSystem {
    fn root_path(&self) -> PathBuf {
        PathBuf::from("/")
    }
}
