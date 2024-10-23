#[cfg(test)]
pub mod tests {
    use std::{
        fs::{self},
        path::PathBuf,
    };

    use crate::utils::start;

    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 200;

    #[test]
    fn test_logo_examples_success() {
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
        file_inputs.sort();
        file_outputs.sort();
        for (i, file) in file_inputs.iter().enumerate() {
            let image_path: PathBuf = "result.svg".into();
            if let Ok(exp) = fs::read_to_string(file_outputs[i].clone()) {
                if exp.trim() != "Error" {
                    let _ = start(file.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
                    if let Ok(out) = fs::read_to_string(image_path) {
                        assert_eq!(out, exp);
                    }
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn example_too_many_args_err() {
        let file_name: PathBuf = "logo_examples/1_09_too_many_args_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_too_many_args_2_err() {
        let file_name: PathBuf = "logo_examples/1_10_too_many_args_2_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_wrong_type_err() {
        let file_name: PathBuf = "logo_examples/1_11_wrong_type_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_1_12_not_enough_args_err() {
        let file_name: PathBuf = "logo_examples/1_12_not_enough_args_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_1_13_wrong_type_err() {
        let file_name: PathBuf = "logo_examples/1_13_wrong_type_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_2_03_missing_var_err() {
        let file_name: PathBuf = "logo_examples/2_03_missing_var_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_2_06_unknownaddassign_err() {
        let file_name: PathBuf = "logo_examples/2_06_unknownaddassign_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_3_07_invalid_if_err() {
        let file_name: PathBuf = "logo_examples/3_07_invalid_if_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_3_08_invalid_while_err() {
        let file_name: PathBuf = "logo_examples/3_08_invalid_while_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_5_03_invalid_err() {
        let file_name: PathBuf = "logo_examples/5_03_invalid_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn example_5_04_invalid_end_err() {
        let file_name: PathBuf = "logo_examples/5_04_invalid_end_err.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn invalid_var_name() {
        let file_name: PathBuf = "tests/invalid_var_name.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }

    #[test]
    #[should_panic]
    fn divide_by_0() {
        let file_name: PathBuf = "tests/divide_by_0.lg".into();
        let image_path: PathBuf = "result.svg".into();
        let _ = start(file_name.to_path_buf(), image_path.clone(), WIDTH, HEIGHT);
    }
}
