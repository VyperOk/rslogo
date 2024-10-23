// src/lib.rs

// Include the code from main.rs for testing
// Writing tests
#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use crate::utils::start;
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 200;

    #[test]
    fn test_add() {
        let file_path: PathBuf = "test.lg".into();
        let image_path: PathBuf = "test.png".into();
        assert_eq!(start(file_path, image_path, WIDTH, HEIGHT), Ok(()))
    }
}
