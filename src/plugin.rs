use std::path::{Path, PathBuf};
use std::io::{Result};
use std::fs::{read_dir, File};
use zip::ZipArchive;

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginMeta {
    name: String,
    author: String,
    pub version: u32,
}

#[derive(Debug)]
pub struct Plugin  {
    pub meta: PluginMeta,
    pub zip_path: PathBuf
}

impl PluginMeta {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn set_author(&mut self, author: &str) {
        self.author = author.to_owned();
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }
}

impl PartialEq for PluginMeta {
    fn eq(&self, other: &PluginMeta) -> bool {
        self.name == other.name &&
            self.author == other.author
    }
}

impl Plugin {
    fn new(meta: PluginMeta, zip_path: PathBuf) -> Plugin {
        Plugin {meta: meta, zip_path: zip_path}
    }
}

pub fn scan<P: AsRef<Path>>(base_dir: P) -> Result<Vec<Plugin>> {
    let iter = read_dir(base_dir)?;
    let mut found = Vec::new();
    for e in iter {
        let path: PathBuf = match e {
            Ok(e) => e.path(),
            Err(e) => return Err(e)
        };

        let path_str = path.to_str().expect("path not unicode");
        if !path_str.ends_with(".zip") {
            continue;
        }

        let file = File::open(&path)?;
        let mut zip_file = ZipArchive::new(file)?;

        let manifest_file = match zip_file.by_name("meta.json") {
            Ok(f) => f,
            _ => continue
        };

        let meta: PluginMeta = serde_json::from_reader(manifest_file)?;
        let mut plugin = Plugin::new(meta, path);
        found.push(plugin);
    }
    Ok(found)
}
