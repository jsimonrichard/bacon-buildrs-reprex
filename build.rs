use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  let mut file = File::create("./src/mymod.rs")?;
  file.write_all(b"pub fn hello_world() {println!(\"Hello world!\")}")?;
  Ok(())
}