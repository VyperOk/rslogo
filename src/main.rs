use clap::Parser;
use parse::{parse_commands, Command};
use unsvg::Image;
mod utils;
mod parse;

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

    let image = Image::new(width, height);

    let commands = parse_commands(&file_path)?;
    execute_commands(&commands, &image); 
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



fn execute_commands(commands: &[Command], image: &Image) {
    for command in commands {
        match command {
            Command::PENUP => println!("PENUP"),
            Command::PENDOWN => println!("PENDOWN"),
            Command::FORWARD(arg) => println!("FORWARD({arg})"),
            Command::BACK(arg) => println!("BACK({arg})"),
            Command::LEFT(arg) => println!("LEFT({arg})"),
            Command::RIGHT(arg) => println!("RIGHT({arg})"),
            Command::SETPENCOLOR(arg) => println!("SETPENCOLOR({arg})"),
            Command::TURN(arg) => println!("TURN({arg})"),
            Command::SETHEADING(arg) => println!("SETHEADING({arg})"),
            Command::SETX(arg) => println!("SETX({arg})"),
            Command::SETY(arg) => println!("SETY({arg})"),
        }
    }
}
