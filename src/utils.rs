pub fn exit_with_error(line_number: usize, message: String) {
  eprintln!("Line {line_number}: {}", message);
  std::process::exit(1);
}