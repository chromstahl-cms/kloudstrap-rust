use std::io::{Write, Result};
use std::fs::{File, read_to_string};
use std::path::Path;

extern crate zip;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod plugin;
use plugin::{Plugin, scan};

fn main() -> Result<()> {
    let plugins = scan("./plugins")?;
    for p in plugins {
        print!("{:?}", p)
    }
    Ok(())
}
