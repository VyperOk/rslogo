use std::{collections::VecDeque, fs, path::Path, str::SplitWhitespace};

use crate::utils::{exit_with_error, is_valid_value, Command, Expression};

pub fn parse_commands(file_path: &Path) -> Result<Vec<Command>, ()> {
    let file = fs::read_to_string(file_path);
    let mut commands: Vec<Command> = Vec::new();
    match file {
        Ok(lines_string) => {
            let lines: Vec<String> = lines_string.lines().map(|line| line.to_string()).collect();
            let mut expected_token = ExpectedToken { braces: 0, end: 0 };
            parse_lines(lines, 0, &mut commands, &mut expected_token);
        }
        Err(e) => {
            exit_with_error(format!("Error: {e}"));
        }
    }
    Ok(commands)
}

#[derive(Clone, Copy)]
struct ExpectedToken {
    braces: usize,
    end: usize,
}

fn parse_lines(
    lines: Vec<String>,
    mut i: usize,
    commands: &mut Vec<Command>,
    expected_token: &mut ExpectedToken,
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
                    get_args(cmd, Some(0), &mut words, &mut args);
                    commands.push(Command::PenUp);
                }
                "PENDOWN" => {
                    get_args(cmd, Some(0), &mut words, &mut args);
                    commands.push(Command::PenDown);
                }
                "FORWARD" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::Forward(args[0].clone()));
                }
                "BACK" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::Back(args[0].clone()));
                }
                "LEFT" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::Left(args[0].clone()));
                }
                "RIGHT" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::Right(args[0].clone()));
                }
                "SETPENCOLOR" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::SetPenColor(args[0].clone()));
                }
                "TURN" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::Turn(args[0].clone()));
                }
                "SETHEADING" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::SetHeading(args[0].clone()));
                }
                "SETX" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::SetX(args[0].clone()));
                }
                "SETY" => {
                    get_args(cmd, Some(1), &mut words, &mut args);
                    commands.push(Command::SetY(args[0].clone()));
                }
                "MAKE" => {
                    get_args(cmd, Some(2), &mut words, &mut args);
                    commands.push(Command::Make((args[0].clone(), args[1].clone())));
                }
                "ADDASSIGN" => {
                    get_args(cmd, Some(2), &mut words, &mut args);
                    commands.push(Command::AddAssign((args[0].clone(), args[1].clone())));
                }
                "]" => {
                    get_args(cmd, Some(0), &mut words, &mut args);
                    if expected_token.braces > 0 {
                        expected_token.braces -= 1;
                    } else {
                        exit_with_error("Error: unmatched ] command".to_string());
                    }
                    return i + 1;
                }
                "IF" => {
                    get_args(cmd, None, &mut words, &mut args);
                    check_has_left_bracket(cmd, &mut args);
                    let mut if_commands = Vec::new();
                    expected_token.braces += 1;
                    i = parse_lines(lines.clone(), i + 1, &mut if_commands, expected_token) - 1;
                    commands.push(Command::If((args[0].clone(), if_commands)));
                }
                "WHILE" => {
                    get_args(cmd, None, &mut words, &mut args);
                    check_has_left_bracket(cmd, &mut args);
                    let mut while_commands = Vec::new();
                    expected_token.braces += 1;
                    i = parse_lines(lines.clone(), i + 1, &mut while_commands, expected_token) - 1;
                    commands.push(Command::While((args[0].clone(), while_commands)));
                }
                "TO" => {
                    if let Some(var_name) = words.next() {
                        if !is_valid_value(var_name) {
                            get_args(cmd, None, &mut words, &mut args);
                            let mut to_commands = Vec::new();
                            expected_token.end += 1;
                            i = parse_lines(lines.clone(), i + 1, &mut to_commands, expected_token)
                                - 1;
                            commands.push(Command::To((var_name.to_string(), args, to_commands)));
                        } else {
                            exit_with_error(format!(
                                "Error: Procedure {var_name} must have valid name"
                            ));
                        }
                    }
                }
                "END" => {
                    get_args(cmd, Some(0), &mut words, &mut args);
                    if expected_token.end > 0 {
                        expected_token.end -= 1;
                    } else {
                        exit_with_error("Error: unmatched end command".to_string());
                    }
                    return i + 1;
                }
                _ => {
                    get_args(cmd, None, &mut words, &mut args);
                    commands.push(Command::Procedure((cmd.to_string(), args)));
                }
            }
        }
        i += 1;
    }
    if expected_token.braces != 0 {
        exit_with_error("Error: Unclosed brace".to_string());
    } else if expected_token.end != 0 {
        exit_with_error("Error: Unclosed procedure".to_string());
    }
    i
}

fn get_args(
    cmd: &str,
    arg_size: Option<usize>,
    words: &mut SplitWhitespace<'_>,
    args: &mut Vec<Expression>,
) {
    let remaining_args: Vec<&str> = words.collect();
    match arg_size {
        Some(arg_count) => {
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
        None => {
            let tokens = &mut VecDeque::from(remaining_args);
            while !tokens.is_empty() {
                if let Some(expression) = Expression::from_tokens(tokens) {
                    args.push(expression);
                }
            }
        }
    }
}

fn check_has_left_bracket(cmd: &str, args: &mut Vec<Expression>) {
    if args.len() != 2 {
        exit_with_error(format!("Error: incorrect expression in '{cmd}' command"))
    }
    if let Some(last) = args.last() {
        match last {
            Expression::Value(elem) => {
                if elem != "[" {
                    exit_with_error("Error: Missing starting brace".to_string());
                } else {
                    args.pop();
                }
            }
            _ => exit_with_error("Error: Missing starting brace".to_string()),
        }
    }
}
