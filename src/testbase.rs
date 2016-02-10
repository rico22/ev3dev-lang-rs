use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

extern crate tempdir;

use system::SystemShim;

pub struct TestSystem {
    pub dir: tempdir::TempDir,
}

impl SystemShim for TestSystem {
    fn root_path(&self) -> PathBuf { self.dir.path().to_path_buf() }
}

pub trait TestCase {
    fn setup(&mut self);
}

pub fn init_file(path: &PathBuf, name: &str, value: &[u8]) {
    let fname = path.join(name);
    println!("fname {}", fname.display());
    File::create(&fname).and_then(|mut f| f.write_all(value))
        .expect("bad write");
}

macro_rules! test {
    ($name:ident $fixture:ident $expr:expr) => (
        #[test]
        fn $name() {
            let mut $fixture = TestSystem {
                dir: tempdir::TempDir::new("").expect("bad tempdir")
            };
            $fixture.setup();
            $expr;
        }
    )
}
