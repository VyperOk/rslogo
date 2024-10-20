use std::{fs, path::Path, str::SplitWhitespace};

use crate::utils::{exit_with_error, QUERIES, VALUE_PREFIXES};

#[derive(Debug)]
pub enum Command {
    PenUp,
    PenDown,
    Forward(String),
    Back(String),
    Left(String),
    Right(String),
    SetPenColor(String),
    Turn(String),
    SetHeading(String),
    SetX(String),
    SetY(String),
    Make((String, String)),
    AddAssign((String, String)),
}

/// After adding every command must add in here
/// returns Command enum associated with string iff value is correct
/// otherwise returns None 

// May need to change all Commands to take a string and then in execute do the parsing 
// Otherwise idk how I am going to implement queries
impl Command {
  fn from_str(cmd: &str, args: Vec<String>) -> Option<Command> {
    match cmd {
      "PENUP" => {
        if args.is_empty() {
          Some(Command::PenUp)
        } else {
          None
        }
      }, 
      "PENDOWN" => {
        if args.is_empty() {
          Some(Command::PenDown)
        } else {
          None
        }
      },
      "FORWARD" => args.first().map(|value| Command::Forward(value.clone())),
      "BACK" => args.first().map(|value| Command::Back(value.clone())),
      "LEFT" => args.first().map(|value| Command::Left(value.clone())),
      "RIGHT" => args.first().map(|value| Command::Right(value.clone())),
      "SETPENCOLOR" => args.first().map(|value| Command::SetPenColor(value.clone())),
      "TURN" => args.first().map(|value| Command::Turn(value.clone())),
      "SETHEADING" => args.first().map(|value| Command::SetHeading(value.clone())),
      "SETX" => args.first().map(|value| Command::SetX(value.clone())),
      "SETY" => args.first().map(|value| Command::SetY(value.clone())),
      "MAKE" => Some(Command::Make((args[0].clone(), args[1].clone()))),
      "ADDASSIGN" => Some(Command::AddAssign((args[0].clone(), args[1].clone()))),
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
            "MAKE" => parse_command(formatted_cmd.as_str(), 2, &mut words, commands, i + 1),
            "ADDASSIGN" => parse_command(formatted_cmd.as_str(), 2, &mut words, commands, i + 1),
            _ => {
                exit_with_error(format!("Line {}: Error: Unrecognised command: {cmd}", i + 1));
            }
        }
    }
}
}

/// Handles errors when reading individual command and pushes only successful cases to commands
fn parse_command(cmd: &str, arg_size: usize,  words: &mut SplitWhitespace<'_>, commands: &mut Vec<Command>, line_number: usize) {
  let remaining_args: Vec<&str> = words.collect();
  if remaining_args.len() != arg_size {
    exit_with_error(format!("Line {}: Error: {} command has an invalid amount of arguments. Received {} arguments, expected {}", line_number, cmd, remaining_args.len(), arg_size));
  }
  let mut args: Vec<String> = Vec::new();
  for arg in remaining_args.clone() {
    if VALUE_PREFIXES.iter().any(|&prefix| arg.to_uppercase().starts_with(prefix)) || QUERIES.iter().any(|&query| arg.to_uppercase() == query) {
      args.push(arg.to_uppercase());
    } else {
        exit_with_error(format!("Line {}: Error: '{}' command invalid argument", line_number, cmd));
    }
  }
  match Command::from_str(cmd, args) {
    Some(result) => commands.push(result),
    None => exit_with_error(format!("Line {}: Error: {} command has invalid arg", line_number, cmd)),
  }
}