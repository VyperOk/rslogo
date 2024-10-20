use unsvg::{get_end_coordinates, Color, Image, COLORS};

use crate::{parse::Command, utils::{exit_with_error, QUERIES, VALUE_PREFIXES}};

trait Executable {
  fn execute(&self, turtle: &mut Turtle, image: &mut Image);
}

struct Turtle {
  pen_down: bool,
  color: Color,
  heading: i32,
  pos_x: i32,
  pos_y: i32,
  variables: Vec<Variable>,
}

#[derive(Debug)]
struct Variable {
  name: String,
  value: String,
}

impl Executable for Command {
  fn execute(&self, turtle: &mut Turtle, image: &mut Image) {
    match self {
      Command::PenUp => turtle.pen_down = false,
      Command::PenDown => turtle.pen_down = true,
      Command::Forward(value) => {
        if let Some(length) = get_value_from_string::<i32>(turtle, value.to_string()) {
          pen_move(image, turtle, turtle.heading, length);
        }
      }
      Command::Back(value) => {
        if let Some(length) = get_value_from_string::<i32>(turtle, value.to_string()) {
          pen_move(image, turtle, turtle.heading + 180, length);
        }
      }
      Command::Left(value) => {
        if let Some(length) = get_value_from_string::<i32>(turtle, value.to_string()) {
          pen_move(image, turtle, turtle.heading - 90, length);
        }
      },
      Command::Right(value) => {
        if let Some(length) = get_value_from_string::<i32>(turtle, value.to_string()) {
          pen_move(image, turtle, turtle.heading + 90, length);
        }
      },
      Command::SetPenColor(value) => {
        if let Some(color_index) = get_value_from_string::<usize>(turtle, value.to_string()) {
          if color_index < COLORS.len() {
            turtle.color = COLORS[color_index];
          } else {
            exit_with_error(format!("Error: Out of bounds access {} is too big. Must be less than {}", color_index, COLORS.len()));
          }
        }
      },
      Command::Turn(value) => {
        if let Some(degrees) = get_value_from_string::<i32>(turtle, value.to_string()) {
          turtle.heading += degrees;
        }
      },
      Command::SetHeading(value) => {
        if let Some(degrees) = get_value_from_string::<i32>(turtle, value.to_string()) {
          turtle.heading = degrees;
        }
      },
      Command::SetX(value) => {
        if let Some(x) = get_value_from_string::<i32>(turtle, value.to_string()) {
          turtle.pos_x = x;
        }
      },
      Command::SetY(value) => {
        if let Some(y) = get_value_from_string::<i32>(turtle, value.to_string()) {
          turtle.pos_y = y;
        }
      },
      Command::Make((name, value)) => {
        if let Some(validated_name) = get_value_from_string::<String>(turtle, name.to_string()) {
          if let Some(validated_value) = get_value_from_string::<String>(turtle, value.to_string()) {
            if let Some(existing_variable) = turtle.variables.iter_mut().find(|v| v.name == validated_name) {
              existing_variable.value = validated_value;
            } else {
              turtle.variables.push(Variable { name: validated_name, value: validated_value });
            }
          }
        }
      },
        Command::AddAssign((name, value)) => {
          if let Some(validated_name) =  get_value_from_string::<String>(turtle, name.to_string()) {
            if let Some(validated_value) = get_value_from_string::<i32>(turtle, value.to_string()) {
              if let Some(existing_variable) = turtle.variables.iter_mut().find(|v| v.name == validated_name) {
                if let Ok(curr_value) = existing_variable.value.parse::<i32>() {
                  existing_variable.value = (curr_value + validated_value).to_string()
                } else {
                  exit_with_error(format!("Error: cannot add parse variable value to integer"));
                }
              }
            }
          }
        },
    }
  }
}

// Must check this works properly
fn get_value_from_string<T: std::str::FromStr>(turtle: &mut Turtle, str: String) -> Option<T> {
  for &prefix in VALUE_PREFIXES.iter() {
    if str.starts_with(prefix) {
      match prefix {
        "\"" => {
          if let Some(stripped) = str.strip_prefix("\"") {
            match T::from_str(stripped) {
              Ok(res) => return Some (res),
              Err(_) => exit_with_error(format!("Error: unable to parse value {}", str)),
            }
          }
        },
        ":" => {
          if let Some(stripped) = str.strip_prefix(":") {
            if let Some(variable) = turtle.variables.iter().find(|variable| variable.name == stripped) {
              match T::from_str(&variable.value) {
                Ok(res) => return Some(res),
                Err(_) => exit_with_error(format!("Error: unable to parse value {}", str)),
              }
            } else {
              exit_with_error(format!("Error: Variable not found"));
            }
          }
        },
        _ => todo!("Add value prefixes and logic here")
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
        },
        _ => todo!("Add queries and logic here")
      }
    }
  }
  None
}

fn pen_move(image: &mut Image, turtle: &mut Turtle, heading: i32, length: i32) {
  let result: Result<(i32, i32), String> = if turtle.pen_down {
    image.draw_simple_line(turtle.pos_x, turtle.pos_y, heading, length, turtle.color)
  } else {
    Ok(get_end_coordinates(turtle.pos_x, turtle.pos_y, heading, length))
  };
  match result {
    Ok((x,y )) => {
      turtle.pos_x = x;
      turtle.pos_y = y;
    },
    Err(e) => {
      eprintln!("{e}");
    }
  }
}


pub fn execute_commands(commands: &[Command], image: &mut Image) {
  let dimensions = image.get_dimensions();
  let (x, y) = (dimensions.0 as i32, dimensions.1 as i32);
  let mut turtle = Turtle {pen_down: false, color: COLORS[7], heading: 0, pos_x: x / 2, pos_y: y / 2, variables: Vec::new() };
  for command in commands {
    command.execute(&mut turtle, image);
  }
}