use crate::prelude::std::all::*;
use crate::state;
pub fn init()-> io::Result<state::buffer::Buffer>{
  let mut buffer = crate::state::buffer::Buffer::new();
  let path = env::home_dir()
    .expect("unable to find home dir.")
    .join("impl")
    .join("rust")
    .join("exec")
    .join("src")
    .join("txt.txt");
  buffer.read(path)?;
  Ok(buffer)
}
