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
    MAKE,
}

#[derive(Debug, Clone)]
struct Variable {
  name: String,
  value: String,
}

/// After adding every command must add in here
/// returns Command enum associated with string iff value is correct
/// otherwise returns None 

// need to change value to array
impl Command {
  fn from_str(cmd: &str, args: Vec<String>, variables: &mut Vec<Variable>) -> Option<Command> {
    match cmd {
      "PENUP" => {
        if args.is_empty() {
          Some(Command::PENUP)
        } else {
          None
        }
      }, 
      "PENDOWN" => {
        if args.is_empty() {
          Some(Command::PENDOWN)
        } else {
          None
        }
      },
      "FORWARD" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::FORWARD(val)),
            Err(_) => None
          }
        } else {
          None
        }
      },
      "BACK" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::BACK(val)),
            Err(_) => None
          }
        } else {
          None
        }
      }
      "LEFT" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::LEFT(val)),
            Err(_) => None
          }
        } else {
          None
        }
      }
      "RIGHT" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::RIGHT(val)),
            Err(_) => None
          }
        } else {
          None
        }
      }
      "SETPENCOLOR" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::SETPENCOLOR(val)),
            Err(_) => None
          }
        } else {
          None
        }
      }
      "TURN" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::TURN(val)),
            Err(_) => None
          }
        } else {
          None
        }
      }
      "SETHEADING" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::SETHEADING(val)),
            Err(_) => None
          }
        } else {
          None
        }
      }
      "SETX" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::SETX(val)),
            Err(_) => None
          }
        } else {
          None
        }
      }
      "SETY" => {
        if let Some(value) = args.get(0) {
          match value.parse::<_>() {
            Ok(val) => Some(Command::SETY(val)),
            Err(_) => None
          }
        } else {
          None
        }
      },
      "MAKE" => {
        let mut existing_variable = variables.iter_mut().filter(|variable| variable.name == args[0]).collect::<Vec<_>>();
        if !existing_variable.is_empty() {
          existing_variable[0].value = args[1].to_string();
        }
        variables.push(Variable { name: args[0].to_string(), value: args[1].to_string()});
        Some(Command::MAKE)
      }
      _ => None
    }
  }
}

/// Opens File and reads lines, then sends to parse_lines to further interpret
pub fn parse_commands(file_path: &Path) -> Result<Vec<Command>, ()> {
  let file = fs::read_to_string(file_path);
  let mut commands: Vec<Command> = Vec::new();
  let mut variables: Vec<Variable> = Vec::new();
  match file {
      Ok(lines_string) => {
          let lines: std::str::Split<'_, &str> = lines_string.split("\n");
          parse_lines(lines, &mut commands, &mut variables);
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
fn parse_lines(lines: std::str::Split<'_, &str>, commands: &mut Vec<Command>, variables: &mut Vec<Variable>) {
  for (i, line) in lines.enumerate() {
    let trimmed_line = line.trim();
    if trimmed_line.starts_with("//") {
        continue;
    }
    let mut words = trimmed_line.split_whitespace();
    if let Some(cmd) = words.next() {
        let formatted_cmd = cmd.to_ascii_uppercase();
        match formatted_cmd.as_str() {
            "PENUP" => parse_command(formatted_cmd.as_str(), 0, &mut words, commands, variables, i + 1),
            "PENDOWN" => parse_command(formatted_cmd.as_str(), 0, &mut words, commands, variables, i + 1),
            "FORWARD" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, variables, i + 1),
            "BACK" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, variables, i + 1),
            "LEFT" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, variables, i + 1),
            "RIGHT" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, variables, i + 1),
            "SETPENCOLOR" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, variables, i + 1),
            "TURN" => parse_command(formatted_cmd.as_str(),1,  &mut words, commands, variables, i + 1),
            "SETHEADING" => parse_command(formatted_cmd.as_str(),1,  &mut words, commands, variables, i + 1),
            "SETX" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, variables, i + 1),
            "SETY" => parse_command(formatted_cmd.as_str(), 1, &mut words, commands, variables, i + 1),
            "MAKE" => parse_command(formatted_cmd.as_str(), 2, &mut words, commands, variables, i + 1),
            _ => {
                exit_with_error(i + 1, format!("Error: Unrecognised command: {cmd}"));
            }
        }
    }
}
}

/// Handles errors when reading individual command and pushes only successful cases to commands
fn parse_command(cmd: &str, arg_size: usize,  words: &mut SplitWhitespace<'_>, commands: &mut Vec<Command>, variables: &mut Vec<Variable>, line_number: usize) {
  let remaining_args: Vec<&str> = words.collect();
  if remaining_args.len() != arg_size {
    exit_with_error(line_number, format!("Error: {} command has an invalid amount of arguments. Received {} arguments, expected {}", cmd, remaining_args.len(), arg_size));
  }
  // This could be my arg validation
  let mut args: Vec<String> = Vec::new();
  for arg in remaining_args.clone() {
    if arg.starts_with('"') {
      args.push(arg[1..].to_string());
    } else if arg.starts_with(":")  {
      let res: Vec<_> = variables.into_iter().filter(|variable| variable.name == arg[1..]).collect();
      if res.len() == 0 {
        exit_with_error(line_number, format!("Error: '{}' command invalid argument, variable is undefined", cmd));
      }
      let value = res[0].value.clone();
      args.push(value);
    } else {
      exit_with_error(line_number, format!("Error: '{}' command invalid argument, must start with \" or :", cmd));
    }
  }
  match Command::from_str(cmd, args, variables) {
    Some(result) => commands.push(result),
    None => exit_with_error(line_number, format!("Error: {} command has invalid arg", cmd)),
  }
}