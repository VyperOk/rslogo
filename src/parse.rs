use std::{collections::VecDeque, fs, path::Path, str::SplitWhitespace};

use crate::utils::{exit_with_error, Command, Expression};

/// Opens File and reads lines, then sends to parse_lines to further interpret
pub fn parse_commands(file_path: &Path) -> Result<Vec<Command>, ()> {
    let file = fs::read_to_string(file_path);
    let mut commands: Vec<Command> = Vec::new();
    match file {
        Ok(lines_string) => {
            let lines: Vec<String> = lines_string.lines().map(|line| line.to_string()).collect();
            parse_lines(lines, 0, &mut commands, &mut 0);
        }
        Err(e) => {
            exit_with_error(format!("Error: {e}"));
        }
    }
    Ok(commands)
}

/// alters commands: Vec.
/// takes raw lines from file and filters out comments and trims whitespace on either end
/// Must add string match when adding new command
fn parse_lines(
    lines: Vec<String>,
    mut i: usize,
    commands: &mut Vec<Command>,
    expected_braces: &mut usize,
) -> usize {
    while let Some(line) = lines.get(i) {
        let trimmed_line = line.trim();
        if trimmed_line.starts_with("//") {
            i += 1;
            continue;
        }
        let mut words = trimmed_line.split_whitespace();
        if let Some(cmd) = words.next() {
            let formatted_cmd = cmd.to_ascii_uppercase();
            let mut args: Vec<Expression> = Vec::new();
            match formatted_cmd.as_str() {
                "PENUP" => {
                    get_args(formatted_cmd.as_str(), Some(0), &mut words, &mut args);
                    commands.push(Command::PenUp);
                }
                "PENDOWN" => {
                    get_args(formatted_cmd.as_str(), Some(0), &mut words, &mut args);
                    commands.push(Command::PenDown);
                }
                "FORWARD" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::Forward(args[0].clone()));
                }
                "BACK" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::Back(args[0].clone()));
                }
                "LEFT" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::Left(args[0].clone()));
                }
                "RIGHT" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::Right(args[0].clone()));
                }
                "SETPENCOLOR" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::SetPenColor(args[0].clone()));
                }
                "TURN" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::Turn(args[0].clone()));
                }
                "SETHEADING" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::SetHeading(args[0].clone()));
                }
                "SETX" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::SetX(args[0].clone()));
                }
                "SETY" => {
                    get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                    commands.push(Command::SetY(args[0].clone()));
                }
                "MAKE" => {
                    get_args(formatted_cmd.as_str(), Some(2), &mut words, &mut args);
                    commands.push(Command::Make((args[0].clone(), args[1].clone())));
                }
                "ADDASSIGN" => {
                    get_args(formatted_cmd.as_str(), Some(2), &mut words, &mut args);
                    commands.push(Command::AddAssign((args[0].clone(), args[1].clone())));
                }
                "]" => {
                    get_args(formatted_cmd.as_str(), Some(0), &mut words, &mut args);
                    *expected_braces -= 1;
                    return i + 1;
                }
                "IF" => {
                    get_args(formatted_cmd.as_str(), None, &mut words, &mut args);
                    let mut if_commands = Vec::new();
                    *expected_braces += 1;
                    i = parse_lines(lines.clone(), i + 1, &mut if_commands, expected_braces) - 1;
                    commands.push(Command::If((args[0].clone(), if_commands)));
                }
                "WHILE" => {
                    get_args(formatted_cmd.as_str(), None, &mut words, &mut args);
                    let mut if_commands = Vec::new();
                    *expected_braces += 1;
                    i = parse_lines(lines.clone(), i + 1, &mut if_commands, expected_braces) - 1;
                    commands.push(Command::While((args[0].clone(), if_commands)));
                }
                _ => {
                    exit_with_error(format!("Error: Unrecognised command: {cmd}"));
                }
            }
        }
        i += 1;
    }
    if *expected_braces != 0 {
        exit_with_error("Error: Unclosed brace".to_string());
    }
    i
}

/// Handles errors when reading individual command and returns args for each command
fn get_args(
    cmd: &str,
    arg_size: Option<usize>,
    words: &mut SplitWhitespace<'_>,
    args: &mut Vec<Expression>,
) {
    let remaining_args: Vec<&str> = words.collect();
    match arg_size {
        // Handles case where there is a fixed number of arguments for the command
        Some(arg_count) => {
            // Checks if there is the correct amount of args
            let tokens = &mut VecDeque::from(remaining_args);
            let mut i: usize = 0;
            while i < arg_count {
                if let Some(expression) = Expression::from_tokens(tokens) {
                    args.push(expression);
                }
                i += 1;
            }
            if args.len() + tokens.len() != arg_count {
                exit_with_error(format!("Error: {} command has an invalid amount of arguments. Received {} arguments, expected {}", cmd, args.len() + tokens.len(), arg_count));
            }
        }
        // Handles case where there is not a set number of arguments for the command
        // returns the expression condition
        None => {
            let tokens = &mut VecDeque::from(remaining_args);
            let expression = Expression::from_tokens(tokens);
            if tokens.front() != Some(&"[") {
                exit_with_error("Error: Mismatched arguments in expression".to_string());
            }
            if let Some(validated_expression) = expression {
                args.push(validated_expression);
            }
        }
    }
}
