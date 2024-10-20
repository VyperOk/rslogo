pub fn exit_with_error(message: String) {
  eprintln!("{}", message);
  std::process::exit(1);
}

pub const VALUE_PREFIXES: [&str; 2] = ["\"", ":"];
pub const QUERIES: [&str; 4] = ["XCOR", "YCOR", "HEADING", "COLOR"];