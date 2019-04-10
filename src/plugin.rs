use std::path::{Path, PathBuf};
use std::io::{Result, Error, ErrorKind};
use std::fs::{read_dir, File};
use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginMeta {
    name: String,
    author: String,
    pub version: u32,
}

#[derive(Debug)]
pub struct Plugin  {
    pub meta: PluginMeta,
    pub archive_path: PathBuf
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
    fn new(meta: PluginMeta, archive_path: PathBuf) -> Plugin {
        Plugin {meta: meta, archive_path: archive_path}
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
        if !path_str.ends_with(".tar.gz") {
            continue;
        }

        let tar_gz = File::open(&path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        let entries = archive.entries()?;

        for entry in entries {
            let path = match &entry {
                Ok(e) => e.path(),
                Err(e) => {panic!("FIX")}
            };
            if path?.to_str().expect("path is not unicode") == "meta.json" {
               entry?.unpack("meta.json");
            }
        }
/*
        let meta: PluginMeta = serde_json::from_reader(manifest_file)?;
        let mut plugin = Plugin::new(meta, path);
        found.push(plugin);
        */
    }
    Ok(found)
}
