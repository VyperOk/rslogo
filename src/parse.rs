use std::{collections::VecDeque, fs, path::Path, str::SplitWhitespace};

use crate::utils::{exit_with_error, is_valid_value, Command, Expression};

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
    loop {
        if let Some(line) = lines.get(i) {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("//") {
                i = i + 1;
                continue;
            }
            let mut words = trimmed_line.split_whitespace();
            if let Some(cmd) = words.next() {
                let formatted_cmd = cmd.to_ascii_uppercase();
                match formatted_cmd.as_str() {
                    "PENUP" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(0), &mut words, &mut args);
                        commands.push(Command::PenUp);
                    }
                    "PENDOWN" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(0), &mut words, &mut args);
                        commands.push(Command::PenDown);
                    }
                    "FORWARD" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::Forward(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "BACK" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::Back(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "LEFT" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::Left(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "RIGHT" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::Right(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "SETPENCOLOR" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::SetPenColor(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "TURN" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::Turn(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "SETHEADING" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::SetHeading(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "SETX" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::SetX(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "SETY" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(1), &mut words, &mut args);
                        match &args[0] {
                            Expression::Value(value) => {
                                commands.push(Command::SetY(value.to_string()))
                            }
                            _ => (),
                        }
                    }
                    "MAKE" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(2), &mut words, &mut args);
                        if let Expression::Value(arg1) = &args[0] {
                            if let Expression::Value(arg2) = &args[1] {
                                commands.push(Command::Make((arg1.to_string(), arg2.to_string())));
                            }
                        }
                    }
                    "ADDASSIGN" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(2), &mut words, &mut args);
                        if let Expression::Value(arg1) = &args[0] {
                            if let Expression::Value(arg2) = &args[1] {
                                commands
                                    .push(Command::AddAssign((arg1.to_string(), arg2.to_string())));
                            }
                        }
                    }
                    "]" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), Some(0), &mut words, &mut args);
                        *expected_braces -= 1;
                        return i + 1;
                    }
                    "IF" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), None, &mut words, &mut args);
                        let mut if_commands = Vec::new();
                        *expected_braces += 1;
                        i = parse_lines(lines.clone(), i + 1, &mut if_commands, expected_braces)
                            - 1;
                        commands.push(Command::If((args[0].clone(), if_commands)));
                    }
                    "WHILE" => {
                        let mut args: Vec<_> = Vec::new();
                        get_args(formatted_cmd.as_str(), None, &mut words, &mut args);
                        let mut if_commands = Vec::new();
                        *expected_braces += 1;
                        i = parse_lines(lines.clone(), i + 1, &mut if_commands, expected_braces)
                            - 1;
                        commands.push(Command::While((args[0].clone(), if_commands)));
                    }
                    _ => {
                        exit_with_error(format!("Error: Unrecognised command: {cmd}"));
                    }
                }
            }
            i = i + 1;
        } else {
            break;
        }
    }
    if *expected_braces != 0 {
        exit_with_error(format!("Error: Unclosed brace"));
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
            if remaining_args.len() != arg_count {
                exit_with_error(format!("Error: {} command has an invalid amount of arguments. Received {} arguments, expected {}", cmd, remaining_args.len(), arg_count));
            }
            for arg in remaining_args {
                if is_valid_value(arg) {
                    args.push(Expression::Value(arg.to_string()));
                } else {
                    exit_with_error(format!("Error: '{}' command invalid argument", cmd));
                }
            }
        }
        // Handles case where there is not a set number of arguments for the command
        // returns the expression condition
        None => {
            let mut tokens = &mut VecDeque::from(remaining_args);
            let expression = Expression::from_tokens(&mut tokens);
            if tokens.front() != Some(&"[") {
                exit_with_error(format!("Error: Mismatched arguments in expression"));
            }
            match expression {
                Some(validated_expression) => args.push(validated_expression),
                None => (),
            }
        }
    }
}
