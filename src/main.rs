use std::io::{Write, Result};
use std::fs::{File};
use std::path::Path;
use std::fs;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate flate2;
extern crate tar;


mod plugin;
use plugin::{scan, PluginMeta};
use flate2::read::GzDecoder;
use tar::Archive;

fn main() -> Result<()> {
    let plugins = scan("./plugins")?;
    let lock_file_path = Path::new("./plugin_lockfile.json");
    let plugin_dir = Path::new("./extracted_plugins");

    if !plugin_dir.exists() {
        fs::create_dir(plugin_dir)?;
    }

    if !lock_file_path.exists() {
        let mut the_file = File::create(lock_file_path)?;
        the_file.write(b"[]")?;
    }
    let lock_file = File::open(lock_file_path)?;

    let mut locks: Vec<PluginMeta> = serde_json::from_reader(&lock_file)?;
    let mut new_locks: Vec<PluginMeta> = Vec::new();

    for p in plugins {
        if locks.contains(&p.meta) {
            if let Some(index) = locks.iter().position(| y | *y.to_owned() == p.meta) {
                let lock_version = &locks[index];
                if lock_version.version >= p.meta.version {
                    continue;
                }
            }
        }

        let tar_gz = File::open(&p.archive_path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        let unpack_path = plugin_dir
                       .join(&p.meta.author)
                       .join(&p.meta.name)
                       .join(&p.meta.version.to_string());

        if !unpack_path.exists() {
            fs::create_dir_all(&unpack_path)?;
        }

        archive.unpack(unpack_path)?;

        print!("{:?}", &p);
        new_locks.push(p.meta);
    }

    locks.append(&mut new_locks);
    serde_json::to_writer(lock_file, &locks)?;

    Ok(())
}
