use unsvg::{get_end_coordinates, Image, COLORS};

use crate::utils::{
    exit_with_error, Command, Expression, Turtle, Variable, QUERIES, VALUE_PREFIXES,
};

impl Command {
    fn execute(&self, turtle: &mut Turtle, image: &mut Image) {
        match self {
            Command::PenUp => turtle.pen_down = false,
            Command::PenDown => turtle.pen_down = true,
            Command::Forward(expression) => {
                if let Some(length) = evaluate_expression::<i32>(turtle, expression) {
                    pen_move(image, turtle, turtle.heading, length);
                }
            }
            Command::Back(expression) => {
                if let Some(length) = evaluate_expression::<i32>(turtle, expression) {
                    pen_move(image, turtle, turtle.heading + 180, length);
                }
            }
            Command::Left(expression) => {
                if let Some(length) = evaluate_expression::<i32>(turtle, expression) {
                    pen_move(image, turtle, turtle.heading - 90, length);
                }
            }
            Command::Right(expression) => {
                if let Some(length) = evaluate_expression::<i32>(turtle, expression) {
                    pen_move(image, turtle, turtle.heading + 90, length);
                }
            }
            Command::SetPenColor(expression) => {
                if let Some(color_index) = evaluate_expression::<usize>(turtle, expression) {
                    if color_index < COLORS.len() {
                        turtle.color = COLORS[color_index];
                    } else {
                        exit_with_error(format!(
                            "Error: Out of bounds access {} is too big. Must be less than {}",
                            color_index,
                            COLORS.len()
                        ));
                    }
                }
            }
            Command::Turn(expression) => {
                if let Some(degrees) = evaluate_expression::<i32>(turtle, expression) {
                    turtle.heading += degrees;
                }
            }
            Command::SetHeading(expression) => {
                if let Some(degrees) = evaluate_expression::<i32>(turtle, expression) {
                    turtle.heading = degrees;
                }
            }
            Command::SetX(expression) => {
                if let Some(x) = evaluate_expression::<i32>(turtle, expression) {
                    turtle.pos_x = x;
                }
            }
            Command::SetY(expression) => {
                if let Some(y) = evaluate_expression::<i32>(turtle, expression) {
                    turtle.pos_y = y;
                }
            }
            Command::Make((name, value)) => {
                if let Some(validated_name) = evaluate_expression::<String>(turtle, name) {
                    if let Some(validated_value) = evaluate_expression::<String>(turtle, value) {
                        if let Some(existing_variable) = turtle
                            .variables
                            .iter_mut()
                            .find(|v| v.name == validated_name)
                        {
                            existing_variable.value = validated_value;
                        } else {
                            turtle.variables.push(Variable {
                                name: validated_name,
                                value: validated_value,
                            });
                        }
                    }
                }
            }
            Command::AddAssign((name, value)) => {
                if let Some(validated_name) = evaluate_expression::<String>(turtle, name) {
                    if let Some(validated_value) = evaluate_expression::<i32>(turtle, value) {
                        if let Some(existing_variable) = turtle
                            .variables
                            .iter_mut()
                            .find(|v| v.name == validated_name)
                        {
                            if let Ok(curr_value) = existing_variable.value.parse::<i32>() {
                                existing_variable.value = (curr_value + validated_value).to_string()
                            } else {
                                exit_with_error(
                                    "Error: cannot add parse variable value to integer".to_string(),
                                );
                            }
                        }
                    }
                }
            }
            Command::If((expression, commands)) => {
                if let Some(is_true) = evaluate_expression::<bool>(turtle, expression) {
                    if is_true {
                        execute_commands(turtle, commands, image);
                    }
                }
            }
            Command::While((expression, commands)) => loop {
                if let Some(is_true) = evaluate_expression::<bool>(turtle, expression) {
                    if is_true {
                        execute_commands(turtle, commands, image);
                    } else {
                        break;
                    }
                }
            },
        }
    }
}

fn evaluate_expression<T>(turtle: &mut Turtle, expression: &Expression) -> Option<T>
where
    T: std::str::FromStr + PartialEq + PartialOrd,
{
    match expression {
        Expression::Eq([left, right]) => {
            let left = evaluate_expression::<String>(turtle, left)?;
            let right = evaluate_expression::<String>(turtle, right)?;
            convert_bool_to_t::<T>(left == right)
        }
        Expression::Ne([left, right]) => {
            let left = evaluate_expression::<String>(turtle, left)?;
            let right = evaluate_expression::<String>(turtle, right)?;
            convert_bool_to_t::<T>(left != right)
        }
        Expression::Gt([left, right]) => {
            let left = evaluate_expression::<i32>(turtle, left)?;
            let right = evaluate_expression::<i32>(turtle, right)?;
            convert_bool_to_t::<T>(left > right)
        }
        Expression::Lt([left, right]) => {
            let left = evaluate_expression::<i32>(turtle, left)?;
            let right = evaluate_expression::<i32>(turtle, right)?;
            convert_bool_to_t::<T>(left < right)
        }
        Expression::And([left, right]) => {
            let left = evaluate_expression::<bool>(turtle, left)?;
            let right = evaluate_expression::<bool>(turtle, right)?;
            convert_bool_to_t::<T>(left && right)
        }
        Expression::Or([left, right]) => {
            let left = evaluate_expression::<bool>(turtle, left)?;
            let right = evaluate_expression::<bool>(turtle, right)?;
            convert_bool_to_t::<T>(left || right)
        }
        Expression::Add([left, right]) => {
            let left = evaluate_expression::<i32>(turtle, left)?;
            let right = evaluate_expression::<i32>(turtle, right)?;
            let sum = left + right;
            T::from_str(&sum.to_string()).ok()
        }
        Expression::Subtract([left, right]) => {
            let left = evaluate_expression::<i32>(turtle, left)?;
            let right = evaluate_expression::<i32>(turtle, right)?;
            let difference = left - right;
            T::from_str(&difference.to_string()).ok()
        }
        Expression::Multiply([left, right]) => {
            let left = evaluate_expression::<i32>(turtle, left)?;
            let right = evaluate_expression::<i32>(turtle, right)?;
            let product = left * right;
            T::from_str(&product.to_string()).ok()
        }
        Expression::Divide([left, right]) => {
            let left = evaluate_expression::<i32>(turtle, left)?;
            let right = evaluate_expression::<i32>(turtle, right)?;
            if right != 0 {
                let quotient = left / right;
                T::from_str(&quotient.to_string()).ok()
            } else {
                exit_with_error("Error: cannot divide by 0".to_string());
                None
            }
        }
        Expression::Value(str) => {
            if let Some(value) = get_value_from_string::<T>(turtle, str.clone()) {
                Some(value)
            } else {
                exit_with_error("Error: idk how you got here".to_string());
                None
            }
        }
    }
}

fn convert_bool_to_t<T: std::str::FromStr>(value: bool) -> Option<T> {
    T::from_str(if value { "true" } else { "false" }).ok()
}

fn get_value_from_string<T: std::str::FromStr>(turtle: &mut Turtle, str: String) -> Option<T> {
    for &prefix in VALUE_PREFIXES.iter() {
        if str.starts_with(prefix) {
            match prefix {
                "\"" => {
                    if let Some(stripped) = str.strip_prefix("\"") {
                        match T::from_str(stripped) {
                            Ok(res) => return Some(res),
                            Err(_) => {
                                exit_with_error(format!("Error: unable to parse value {}", str))
                            }
                        }
                    }
                }
                ":" => {
                    if let Some(stripped) = str.strip_prefix(":") {
                        if let Some(variable) = turtle
                            .variables
                            .iter()
                            .find(|variable| variable.name == stripped)
                        {
                            match T::from_str(&variable.value) {
                                Ok(res) => return Some(res),
                                Err(_) => {
                                    exit_with_error(format!("Error: unable to parse value {}", str))
                                }
                            }
                        } else {
                            exit_with_error(format!("Error: Variable '{stripped}' not found"));
                        }
                    }
                }
                _ => todo!("Add value prefixes and logic here"),
            }
        }
    }
    for &query in QUERIES.iter() {
        if str == query {
            match query {
                "XCOR" => return T::from_str(&turtle.pos_x.to_string()).ok(),
                "YCOR" => return T::from_str(&turtle.pos_y.to_string()).ok(),
                "HEADING" => return T::from_str(&turtle.heading.to_string()).ok(),
                "COLOR" => {
                    if let Some(pos) = COLORS.iter().position(|&color| color == turtle.color) {
                        return T::from_str(&pos.to_string()).ok();
                    }
                }
                _ => todo!("Add queries and logic here"),
            }
        }
    }
    None
}

fn pen_move(image: &mut Image, turtle: &mut Turtle, heading: i32, length: i32) {
    let result: Result<(i32, i32), String> = if turtle.pen_down {
        image.draw_simple_line(turtle.pos_x, turtle.pos_y, heading, length, turtle.color)
    } else {
        Ok(get_end_coordinates(
            turtle.pos_x,
            turtle.pos_y,
            heading,
            length,
        ))
    };
    match result {
        Ok((x, y)) => {
            turtle.pos_x = x;
            turtle.pos_y = y;
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }
}

pub fn execute_commands(turtle: &mut Turtle, commands: &[Command], image: &mut Image) {
    for command in commands {
        command.execute(turtle, image);
    }
}
