use std::{fs, path::Path, str::SplitWhitespace};

use crate::utils::exit_with_error;

#[derive(Debug)]
pub enum Command {
    PENUP,
    PENDOWN,
    FORWARD(i32),
    BACK(i32),
    LEFT(i32),
    RIGHT(i32),
    SETPENCOLOR(usize),
    TURN(i32),
    SETHEADING(i32),
    SETX(i32),
    SETY(i32),
}

/// After adding every command must add in here
/// returns Command enum associated with string iff value is correct
/// otherwise returns None
impl Command {
  fn from_str(cmd: &str, value: &str) -> Option<Command> {
    match cmd {
      "PENUP" => {
        if value.is_empty() {
          Some(Command::PENUP)
        } else {
          None
        }
      }, 
      "PENDOWN" => {
        if value.is_empty() {
          Some(Command::PENDOWN)
        } else {
          None
        }
      },
      "FORWARD" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::FORWARD(val))
        } else {
          None
        }
      },
      "BACK" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::BACK(val))
        } else {
          None
        }
      }
      "LEFT" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::LEFT(val))
        } else {
          None
        }
      }
      "RIGHT" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::RIGHT(val))
        } else {
          None
        }
      }
      "SETPENCOLOR" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::SETPENCOLOR(val))
        } else {
          None
        }
      }
      "TURN" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::TURN(val))
        } else {
          None
        }
      }
      "SETHEADING" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::SETHEADING(val))
        } else {
          None
        }
      }
      "SETX" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::SETX(val))
        } else {
          None
        }
      }
      "SETY" => {
        if let Ok(val) = value.parse::<_>() {
          Some(Command::SETY(val))
        } else {
          None
        }
      }
      _ => None
    }
  }
}

/// Opens File and reads lines, then sends to parse_lines to further interpret
pub fn parse_commands(file_path: &Path) -> Result<Vec<Command>, ()> {
  let file = fs::read_to_string(file_path);
  let mut commands: Vec<Command> = Vec::new();
  match file {
      Ok(lines_string) => {
          let lines: std::str::Split<'_, &str> = lines_string.split("\n");
          parse_lines(lines, &mut commands);
          
      },
      Err(e) => {
          println!("{e}")
      }
  }
  Ok(commands)
}

/// alters commands: Vec.
/// takes raw lines from file and filters out comments and trims whitespace on either end
/// Must add string match when adding new command
fn parse_lines(lines: std::str::Split<'_, &str>, commands: &mut Vec<Command>) {
  for (i, line) in lines.enumerate() {
    let trimmed_line = line.trim();
    if trimmed_line.starts_with("//") {
        continue;
    }
    let mut words = trimmed_line.split_whitespace();
    if let Some(cmd) = words.next() {
        let formatted_cmd = cmd.to_ascii_uppercase();
        match formatted_cmd.as_str() {
            "PENUP" => parse_command(formatted_cmd.as_str(), 0, &mut words, commands, i + 1),
            "PENDOWN" => parse_command(formatted_cmd.as_str(), 0, &mut words, commands, i + 1),
            "FORWARD" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, i + 1),
            "BACK" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, i + 1),
            "LEFT" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, i + 1),
            "RIGHT" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, i + 1),
            "SETPENCOLOR" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, i + 1),
            "TURN" => parse_command(formatted_cmd.as_str(),1,  &mut words, commands, i + 1),
            "SETHEADING" => parse_command(formatted_cmd.as_str(),1,  &mut words, commands, i + 1),
            "SETX" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, i + 1),
            "SETY" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, i + 1),
            _ => {
                exit_with_error(i + 1, format!("Error: Unrecognised command: {cmd}"));
            }
        }
    }
}
}

/// Handles errors when reading individual command and pushes only successful cases to commands
fn parse_command(cmd: &str, arg_size: usize,  words: &mut SplitWhitespace<'_>, commands: &mut Vec<Command>, line_number: usize) {
  let remaining_args: Vec<&str> = words.collect();
  if remaining_args.len() != arg_size {
    exit_with_error(line_number, format!("Error: {} command has an invalid amount of arguments. Received {} arguments, expected {}", cmd, remaining_args.len(), arg_size));
  }
  if let Some(arg) = remaining_args.get(0) {
    if arg_size == 0 {
      exit_with_error(line_number, format!("Error: {} command should not have any arguments", cmd));
    }
    if arg.len() > 1 {
      if arg.starts_with('"') {
        match Command::from_str(cmd, &arg[1..]) {
          Some(result) => commands.push(result),
          None => exit_with_error(line_number, format!("Error: {} command has invalid arg", cmd)),
        }
      } else {
        exit_with_error(line_number, format!("Error: '{}' command invalid argument, must start with \" or :", cmd));
      }
    } else {
      exit_with_error(line_number, format!("Error: {} command cannot have empty value for argument", cmd));
    }
  } else {
    if arg_size == 0 {
      match Command::from_str(cmd, "") {
        Some(result) => commands.push(result),
        None => exit_with_error(line_number, format!("Error: '{}' command somehow got here even though there is no arg and there should be no arg", cmd)),
      }
    } else {
      exit_with_error(line_number, format!("Error: '{}' command requires an argument", cmd));
    }
  }
}