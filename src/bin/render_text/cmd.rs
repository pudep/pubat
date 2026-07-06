use crate::app::app;
use crate::state::ViewPort;
use crate::read::read;
use std::error::Error;
use std::path::PathBuf;

use clap::Parser;
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  file_path: String,
}

fn resolve_path(path: &str) -> std::result::Result<PathBuf, String> {
  let expanded_path =
    shellexpand::full(path).map_err(|e| format!("Failed to expand path: `{path}`, : {e}"))?;
  Ok(PathBuf::from(expanded_path.as_ref()))
}

pub fn cli() -> std::result::Result<(), Box<dyn Error>> {
  let cli = Cli::parse();
  let path = resolve_path(&cli.file_path)?;

  match std::fs::metadata(&path) {
    Ok(meta) if meta.is_file() => {
      let buf = read(&path)?;
      let mut viewport = ViewPort::new();
      ratatui::run(|terminal| app(terminal, &buf.rope, &mut viewport, &path))?;
    }
    Ok(meta) if meta.is_dir() => {
      eprintln!("`{}` is a directory", path.display());
      std::process::exit(1);
    }
    Ok(_) => {
      eprintln!("`{}` is not a regular file", path.display());
      std::process::exit(1);
    }

    Err(e) => {
      eprintln!("Cannot access `{}` : {e}", path.display());
      std::process::exit(1);
    }
  }
  Ok(())
}
