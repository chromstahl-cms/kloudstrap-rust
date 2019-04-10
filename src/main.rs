use std::io::{Write, Result};
use std::fs::{File, read_to_string};
use std::path::Path;

mod plugin;
use plugin::{Plugin, scan};

fn main() -> Result<()> {
    let plugins = scan("./plugins")?;
    let mut p = Plugin::default();
    p.set_name("freds_Plugin");
    Ok(())
}
