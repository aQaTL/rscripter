use rscripter::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	log!("Hello, World");
	log!(echo("Hello, World"));

	let name = "Bob";
	log!(echo(format!("Hello, {name}")));

	Ok(())
}
