use unsvg::{get_end_coordinates, Color, Image, COLORS};

use crate::parse::Command;

trait Executable {
  fn execute(&self, turtle: &mut Turtle, image: &mut Image);
}

struct Turtle {
  pen_down: bool,
  color: Color,
  heading: i32,
  pos_x: i32,
  pos_y: i32,
}

impl Executable for Command {
  fn execute(&self, turtle: &mut Turtle, image: &mut Image) {
    match self {
      Command::PENUP => turtle.pen_down = false,
      Command::PENDOWN => turtle.pen_down = true,
      Command::FORWARD(length) => pen_move(image, turtle, turtle.heading, *length),
      Command::BACK(length) => pen_move(image, turtle, turtle.heading + 180, *length),
      Command::LEFT(length) => pen_move(image, turtle, turtle.heading - 90, *length),
      Command::RIGHT(length) => pen_move(image, turtle, turtle.heading + 90, *length),
      Command::SETPENCOLOR(color) => turtle.color = COLORS[*color],
      Command::TURN(value) => turtle.heading += *value,
      Command::SETHEADING(value) => turtle.heading = *value,
      Command::SETX(value) => turtle.pos_x = *value,
      Command::SETY(value) => turtle.pos_y = *value,
      _ => (),
    }
  }
}

fn pen_move(image: &mut Image, turtle: &mut Turtle, heading: i32, length: i32) {
  let result: Result<(i32, i32), String> = if turtle.pen_down == true {
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
  let mut turtle = Turtle {pen_down: false, color: COLORS[7], heading: 0, pos_x: x / 2, pos_y: y / 2 };
  for command in commands {
    command.execute(&mut turtle, image);
  }
}