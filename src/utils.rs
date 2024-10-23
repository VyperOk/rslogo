use std::{collections::VecDeque, path::PathBuf};

use unsvg::{Color, Image, COLORS};

use crate::{execute::execute_commands, parse::parse_commands};

pub const VALUE_PREFIXES: [&str; 2] = ["\"", ":"];
pub const QUERIES: [&str; 4] = ["XCOR", "YCOR", "HEADING", "COLOR"];

#[derive(Debug)]
pub struct Turtle {
    pub(crate) pen_down: bool,
    pub(crate) color: Color,
    pub(crate) heading: i32,
    pub(crate) pos_x: i32,
    pub(crate) pos_y: i32,
    pub(crate) variables: Vec<Variable>,
    pub(crate) procedures: Vec<Procedure>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub(crate) name: String,
    pub(crate) args: Vec<String>,
    pub(crate) commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub enum Command {
    PenUp,
    PenDown,
    Forward(Expression),
    Back(Expression),
    Left(Expression),
    Right(Expression),
    SetPenColor(Expression),
    Turn(Expression),
    SetHeading(Expression),
    SetX(Expression),
    SetY(Expression),
    Make((Expression, Expression)),
    AddAssign((Expression, Expression)),
    If((Expression, Vec<Command>)),
    While((Expression, Vec<Command>)),
    To((String, Vec<Expression>, Vec<Command>)),
    Procedure((String, Vec<Expression>)),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Eq([Box<Expression>; 2]),
    Ne([Box<Expression>; 2]),
    Gt([Box<Expression>; 2]),
    Lt([Box<Expression>; 2]),
    And([Box<Expression>; 2]),
    Or([Box<Expression>; 2]),
    Add([Box<Expression>; 2]),
    Subtract([Box<Expression>; 2]),
    Multiply([Box<Expression>; 2]),
    Divide([Box<Expression>; 2]),
    Value(String),
}

impl Expression {
    pub fn from_tokens(tokens: &mut VecDeque<&str>) -> Option<Self> {
        if let Some(token) = tokens.pop_front() {
            match token.to_uppercase().as_str() {
                "EQ" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Eq([Box::new(left), Box::new(right)]))
                }
                "NE" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Ne([Box::new(left), Box::new(right)]))
                }
                "GT" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Gt([Box::new(left), Box::new(right)]))
                }
                "LT" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Lt([Box::new(left), Box::new(right)]))
                }
                "AND" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::And([Box::new(left), Box::new(right)]))
                }
                "OR" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Or([Box::new(left), Box::new(right)]))
                }
                "+" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Add([Box::new(left), Box::new(right)]))
                }
                "-" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Subtract([Box::new(left), Box::new(right)]))
                }
                "*" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Multiply([Box::new(left), Box::new(right)]))
                }
                "/" => {
                    let left = Expression::from_tokens(tokens)?;
                    let right = Expression::from_tokens(tokens)?;
                    Some(Expression::Divide([Box::new(left), Box::new(right)]))
                }
                "[" => Some(Expression::Value("[".to_string())),
                _ => {
                    if is_valid_value(token) {
                        Some(Expression::Value(token.to_string()))
                    } else {
                        exit_with_error(format!("Error: Invalid value '{token}' in expression"));
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

#[cfg(not(test))]
pub fn exit_with_error(message: String) {
    eprintln!("{}", message);
    std::process::exit(1);
}

#[cfg(test)]
pub fn exit_with_error(message: String) {
    eprintln!("{}", message);
    panic!("{}", message); // Use panic! instead of exit to allow for testing
}

pub fn is_valid_value(str: &str) -> bool {
    if VALUE_PREFIXES.iter().any(|&prefix| str.starts_with(prefix))
        || QUERIES.iter().any(|&query| str.to_uppercase() == query)
    {
        return true;
    }
    false
}

pub fn save_image(image_path: PathBuf, image: Image) {
    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                exit_with_error(format!("Error saving svg: {e}"));
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                exit_with_error(format!("Error saving png: {e}"));
            }
        }
        _ => {
            exit_with_error(format!("File extension not supported"));
        }
    }
}

pub fn start(file_path: PathBuf, image_path: PathBuf, width: u32, height: u32) -> Result<(), ()> {
    let mut image = Image::new(width, height);

    // Create turtle object
    let dimensions = image.get_dimensions();
    let (x, y) = (dimensions.0 as i32, dimensions.1 as i32);
    let mut turtle = Turtle {
        pen_down: false,
        color: COLORS[7],
        heading: 0,
        pos_x: x / 2,
        pos_y: y / 2,
        variables: Vec::new(),
        procedures: Vec::new(),
    };

    let commands = parse_commands(&file_path)?;
    execute_commands(&mut turtle, &commands, &mut image);
    save_image(image_path, image);
    Ok(())
}
