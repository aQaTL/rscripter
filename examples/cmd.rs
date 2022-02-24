use rscripter::*;
use std::process::Child;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut child: Child = log!(cmd!(fork; "xclock",))?;
	cmd!("ping", "-c", "3", "1.1.1.1")?;
	log!(child.kill())?;

	Ok(())
}
