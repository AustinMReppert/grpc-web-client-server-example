use std::{fs};
use std::path::{PathBuf};

// Output may not be shown unless running with -vv.
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let proto_path = PathBuf::from("proto/test.proto");
  let proto = PathBuf::from("proto");

  let output_path = PathBuf::from("output");
  fs::create_dir_all(&output_path).expect("Failed to create output directory");

  tonic_build::configure().out_dir(&output_path)
    .build_server(false)
    .compile(&[proto_path], &[proto]).expect("Failed to compile proto files.");

  Ok(())
}