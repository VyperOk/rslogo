use std::collections::VecDeque;

use unsvg::Color;

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
}

#[derive(Debug)]
pub struct Variable {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Debug)]
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

pub fn exit_with_error(message: String) {
    eprintln!("{}", message);
    std::process::exit(1);
}

pub fn is_valid_value(str: &str) -> bool {
    if VALUE_PREFIXES.iter().any(|&prefix| str.starts_with(prefix))
        || QUERIES.iter().any(|&query| str.to_uppercase() == query)
    {
        return true;
    }
    false
}
