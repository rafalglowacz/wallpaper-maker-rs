use clap::Parser;
use std::fs;
use std::path::Path;
use regex::Regex;
use std::thread;
use std::time::Duration;
use image::{imageops, RgbImage};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    directory: String,
    dest_width: usize,
    dest_height: usize,
    #[arg(default_value = "60")]
    delay: usize,
    #[arg(long, default_value = "false")]
    force: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = get_path(&args);

    println!("Reading directory: {}", path);
    
    if args.delay > 0 {
        println!("Waiting {} seconds...", args.delay);
        thread::sleep(Duration::from_secs(args.delay as u64));
    }

    let image_pattern = Regex::new(r"\.(jpe?g|png|webp)$").unwrap();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            if image_pattern.is_match(path_str) {
                make_wallpaper(path_str, args.dest_width, args.dest_height);
            }
        }
    }

    Ok(())
}

fn make_wallpaper(file_path: &str, dest_width: usize, dest_height: usize) {
    println!("{}", file_path);
    if let Ok(img) = image::open(file_path) {
        let resized = img.resize(
            dest_width as u32,
            dest_height as u32,
            imageops::FilterType::Lanczos3,
        );
        
        let bg = img.resize_exact(
            dest_width as u32,
            dest_height as u32,
            imageops::FilterType::Nearest,
        )
            .fast_blur(25.0);

        let mut final_image = RgbImage::new(dest_width as u32, dest_height as u32);
        imageops::overlay(&mut final_image, &bg.to_rgb8(), 0, 0);

        let x = ((dest_width as i32) - (resized.width() as i32)) / 2;
        let y = ((dest_height as i32) - (resized.height() as i32)) / 2;
        imageops::overlay(&mut final_image, &resized.to_rgb8(), x as i64, y as i64);

        let output_path = format!(
            "/tmp/Tapety/{}", 
            Path::new(file_path).file_name().unwrap().to_str().unwrap()
        );
        
        if let Err(e) = final_image.save(&output_path) {
            eprintln!("Failed to save image {}: {}", output_path, e);
        }
    } else {
        eprintln!("Failed to open image: {}", file_path);
    }
}

fn get_path(args: &Args) -> String {
    let path = args.directory.trim_end_matches('/').to_string();
    shellexpand::tilde(path.as_str()).to_string()
}
