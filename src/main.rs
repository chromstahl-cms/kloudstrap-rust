use std::io::{Write, Result};
use std::fs::{File, read_to_string};
use std::path::Path;
use std::fs;

extern crate zip;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate flate2;
extern crate tar;


mod plugin;
use plugin::{Plugin, scan, PluginMeta};
use zip::ZipArchive;
use flate2::read::GzDecoder;
use tar::Archive;

fn main() -> Result<()> {
    let plugins = scan("./plugins")?;
    let lock_file = Path::new("./plugin_lockfile.json");
    let plugin_dir = Path::new("./extracted_plugins");

    if !plugin_dir.exists() {
        fs::create_dir(plugin_dir)?;
    }

    if !lock_file.exists() {
        let mut the_file = File::create(lock_file)?;
        the_file.write(b"[]")?;
    }
/*
    let the_file = File::open(lock_file)?;

    let locks: Vec<PluginMeta> = serde_json::from_reader(the_file)?;

    for p in plugins {
        if locks.contains(&p.meta) {
            if let Some(index) = locks.iter().position(| y | *y.to_owned() == p.meta) {
                let lock_version = &locks[index];
                if lock_version.version >= p.meta.version {
                    continue;
                }
            }
        }

        let file = File::open(&p.zip_path)?;
        let mut zip_file = ZipArchive::new(file)?;
        print!("{:?}", p)
    }
    */
    Ok(())
}
