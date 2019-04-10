use std::path::Path;
use std::io::{Result};
use std::fs::read_dir;
use std::fs::DirEntry;

pub struct Plugin  {
    name: String,
    author: String,
    version: String,
}

impl Default for Plugin {
    fn default() -> Self {
        Plugin {
            name: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
        }
    }

}

impl Plugin {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn set_author(&mut self, author: &str) {
        self.author = author.to_owned();
    }

    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_owned();
    }
}

pub fn scan<P: AsRef<Path>>(base_dir: P) -> Result<Vec<Plugin>> {
    let iter = read_dir(base_dir)?;
    for entry in iter {
        let e = match entry {
            Ok(e) => e,
            Err(e) => return Err(e)
        };
        println!("{:?}", e.path())
    }
    Ok(Vec::new())
}
