use image;
use image_utils;
use std::path::Path;
use tempfile;

pub fn resize_image(file_name: String) -> image::DynamicImage {
    let temp_dir = tempfile::tempdir().unwrap();
    let output_file = &temp_dir.path().join(".resized.jpg");

    image_utils::resize(&Path::new(&file_name), 50, 50, &Path::new(output_file)).unwrap();

    image::open(output_file).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    #[test]
    fn test_resize_image() {
        // println!("{:?}", resize_image("src/resize/test_resize.jpg".to_string()).dimensions());
        assert_eq!(
            resize_image("src/resize/test_resize.jpg".to_string()).dimensions(),
            (33, 50)
        );
    }
}
