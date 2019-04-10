use std::path::{Path, PathBuf};
use std::io::{Result};
use std::fs::{read_dir, File};
use zip::ZipArchive;

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginMeta {
    name: String,
    author: String,
    version: String,
}

pub struct Plugin  {
    meta: PluginMeta,
    zip_path: Option<PathBuf>
}

impl PluginMeta {
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

impl Plugin {
    fn with_meta(meta: PluginMeta) -> Plugin {
        Plugin {meta: meta, zip_path: Option::None}
    }
}

pub fn scan<P: AsRef<Path>>(base_dir: P) -> Result<Vec<Plugin>> {
    let iter = read_dir(base_dir)?;
    let mut found = Vec::new();
    for e in iter {
        let path = match e {
            Ok(e) => e.path(),
            Err(e) => return Err(e)
        };

        if !path.ends_with(".zip") {
            continue;
        }

        let file = File::open(&path)?;
        let mut zip_file = ZipArchive::new(file)?;

        let manifest_file = match zip_file.by_name("meta.json") {
            Ok(f) => f,
            _ => continue
        };

        let meta: PluginMeta = serde_json::from_reader(manifest_file)?;
        let mut plugin = Plugin::with_meta(meta);
        plugin.zip_path = Some(path);
        found.push(plugin);
    }
    Ok(found)
}
