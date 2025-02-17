use std::cmp::max;
use std::ffi::OsStr;
use std::time::{Duration, Instant};
use image::{ImageReader};
use image::imageops::FilterType;
use walkdir;
use walkdir::{WalkDir};

fn main() {
    let image_formats = ["png", "jpg", "jpeg"];
    let now = Instant::now();
    let mut local_now: Instant;

    let mut extension_calc = Duration::from_millis(0);
    let mut reader_opening = Duration::from_millis(0);
    let mut image_opening = Duration::from_millis(0);
    let mut image_resizing = Duration::from_millis(0);
    let mut image_saving = Duration::from_millis(0);
    let mut i = 0;

    for e in WalkDir::new("./test_folder").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_dir() {
            println!("{}", e.path().display());
            continue;
        }

        local_now = Instant::now();
        let extension = e.path().extension().unwrap_or(OsStr::new(""));
        extension_calc += local_now.elapsed();

        if image_formats.iter().any(|&format| extension == format) {
            i += 1;
            println!("\t|-- {}", e.path().file_name().unwrap().to_str().unwrap());

            local_now = Instant::now();
            let img_reader = ImageReader::open(e.path()).unwrap();
            reader_opening += local_now.elapsed();

            let (width, height) = img_reader.into_dimensions().unwrap();

            if max(width, height) <= 2000 {
                continue;
            }

            let resize_coefficient: f32 = 2000_f32 / max(width, height) as f32;

            local_now = Instant::now();
            let image = image::open(e.path()).unwrap();
            image_opening += local_now.elapsed();

            local_now = Instant::now();
            let resized_image = image.resize(
                (width as f32 * resize_coefficient) as u32,
                (height as f32 * resize_coefficient) as u32,
                FilterType::Lanczos3);
            image_resizing += local_now.elapsed();

            local_now = Instant::now();
            resized_image.save(e.path()).expect("TODO: panic message");
            image_saving += local_now.elapsed();
        }
    }

    println!("Total images processed:\t{:?}", i);
    println!("Extension calc mean:\t{:?}", extension_calc / i);
    println!("Reader opening mean:\t{:?}", reader_opening / i);
    println!("Image opening mean:\t{:?}", image_opening / i);
    println!("Image resizing mean:\t{:?}", image_resizing / i);
    println!("Image saving mean:\t{:?}", image_saving / i);
    println!("Total:\t{:?}", now.elapsed());
}