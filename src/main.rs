use clap::Parser;
use utils::start;
mod execute;
mod parse;
pub mod tests;
mod utils;

#[derive(Parser)]
struct Args {
    file_path: std::path::PathBuf,

    image_path: std::path::PathBuf,

    height: u32,

    width: u32,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let _ = start(file_path, image_path, width, height);

    Ok(())
}
