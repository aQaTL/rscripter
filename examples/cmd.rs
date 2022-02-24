use std::process::Child;
use rscripter::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut child: Child = log!(cmd!(fork; "xclock",))?;
    log!(cmd!("ping", "-c", "3", "1.1.1.1"))?;
    log!(child.kill())?;

    Ok(())
}
