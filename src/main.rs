use clap::Parser;
use execute::execute_commands;
use parse::parse_commands;
use unsvg::{Image, COLORS};
use utils::Turtle;
mod execute;
mod parse;
mod utils;

/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let mut image = Image::new(width, height);

    let commands = parse_commands(&file_path)?;

    // Create turtle object
    let dimensions = image.get_dimensions();
    let (x, y) = (dimensions.0 as i32, dimensions.1 as i32);
    let mut turtle = Turtle {
        pen_down: false,
        color: COLORS[7],
        heading: 0,
        pos_x: x / 2,
        pos_y: y / 2,
        variables: Vec::new(),
    };
    execute_commands(&mut turtle, &commands, &mut image);

    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}
