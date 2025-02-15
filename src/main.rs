use std::cmp::max;
use std::ffi::OsStr;
use std::time::Instant;
use image::{imageops, ImageReader};
use image::imageops::FilterType;
use walkdir;
use walkdir::{WalkDir};

fn main() {
    let image_formats = ["png", "jpg", "jpeg"];
    let now = Instant::now();
    for e in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        println!("{}", e.path().display());

        let extension = e.path().extension().unwrap_or(OsStr::new(""));

        if e.metadata().unwrap().is_file()
            && image_formats.iter().any(|&format| extension == format) {

            let img_reader = ImageReader::open(e.path()).unwrap();
            let (width, height) = img_reader.into_dimensions().unwrap();

            if max(width, height) < 2000 {
                continue;
            }

            let image = image::open(e.path()).unwrap();

            let resize_coefficient: f32 = 2000_f32 / max(width, height) as f32;
            let resized_image = image.resize(
                (width as f32 * resize_coefficient) as u32,
                (height as f32 * resize_coefficient) as u32,
                FilterType::Lanczos3);

            resized_image.save(e.path()).expect("TODO: panic message");
        }
    }
    println!("Took: {:?}", now.elapsed());
}