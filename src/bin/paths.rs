use std::env::{current_dir, var};
use std::path::PathBuf;
use rscripter::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir: PathBuf = current_dir()?;

    let home = var("HOME")?;
    echo(format!("HOME: {home}"));
    log!(cd(home))?;

    echo("Contents:");
    ls!()?;

    echo(format!("cd {}", current_dir.display()));
    cd(&current_dir)?;

    ls!(&current_dir)?;

    // Notice that path! macro takes anything as long as it implements AsRef<Path>, so you can mix
    // `Path`, `PathBuf`, `&str`, `String`, `&OsStr`, `OsString` etc...
    let this_file: PathBuf = path![current_dir, "src", String::from("Cargo.toml")];
    echo(format!("this_file: {}", this_file.display()));

    Ok(())
}