use std::time::Instant;
use image::imageops::FilterType;
use walkdir;
use walkdir::{WalkDir};

fn main() {
    let image_formats = ["png", "jpg", "jpeg"];
    let now = Instant::now();
    for e in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        println!("{}", e.path().display());
        if e.metadata().unwrap().is_file()
            && image_formats.iter().any(|format| e.path().ends_with(format)) {

            let image = image::open(e.path()).unwrap();

            let width = image.width() as f32;
            let height = image.height() as f32;

            if width.max(height) < 2000_f32 {
                continue;
            }

            let resize_coefficient: f32 = 2000_f32 / width.max(height);


            let resized_image = image.resize(
                (width * resize_coefficient) as u32,
                (height * resize_coefficient) as u32,
                FilterType::Lanczos3);

            let path = e.path().to_str().unwrap();

            // let (path_with_filename, extension) = path.split_at(path.rfind(".").unwrap());
            // for debug purposes
            // let output_path = [path_with_filename, "-v2", extension].concat();
            resized_image.save(path).expect("TODO: panic message");
        }
    }
    println!("Took: {:?}", now.elapsed());
}