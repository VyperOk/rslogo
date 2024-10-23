// src/lib.rs

// Include the code from main.rs for testing
// Writing tests
#[cfg(test)]
pub mod tests {
    use std::{
        fs::{self, read_dir},
        path::{Path, PathBuf},
    };

    use crate::utils::start;
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 200;

    #[test]
    fn test_logo_examples() {
        let logo_examples = fs::read_dir("logo_examples").unwrap();
        let logo_examples_out = fs::read_dir("logo_examples_out").unwrap();
        let mut file_inputs: Vec<_> = Vec::new();
        let mut file_outputs: Vec<_> = Vec::new();
        for path in logo_examples {
            file_inputs.push(path.unwrap().path());
        }
        for path in logo_examples_out {
            file_outputs.push(path.unwrap().path());
        }
        dbg!(file_inputs);
        dbg!(file_outputs);
        let file_path: PathBuf = "test.lg".into();
        let image_path: PathBuf = "test.svg".into();
        assert_eq!(start(file_path, image_path, WIDTH, HEIGHT), Ok(()))
    }
}
