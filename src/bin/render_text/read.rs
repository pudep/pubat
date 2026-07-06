use crate::prelude::normal::*;
use std::path::Path;

pub fn read(path: &Path) -> result::Result<crate::state::buffer::Buffer, Box<dyn Error>> {
  let mut buf = crate::state::buffer::Buffer::new();
  buf.read(path.to_path_buf())?;
  Ok(buf)
}
