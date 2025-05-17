use clap::Parser;
use image::{imageops, RgbImage};
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

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

struct Progress {
    last_update: std::time::Instant,
    last_percentage: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let source = get_source_dir(&args);
    let target = ensure_target_dir(&source)?;

    println!("Reading directory: {}", source);
    
    if args.delay > 0 {
        println!("Waiting {} seconds...", args.delay);
        thread::sleep(Duration::from_secs(args.delay as u64));
    }

    let image_pattern = Regex::new(r"\.(jpe?g|png|webp)$").unwrap();
    let (source_item_count, mut progress) = init_progress(&source)?;

    for (i, entry) in fs::read_dir(source)?.enumerate() {
        let entry = entry?;
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            if image_pattern.is_match(path_str) {
                make_wallpaper(path_str, &target, args.dest_width, args.dest_height, args.force);

                let percentage = (i + 1) * 100 / source_item_count;
                let elapsed = progress.last_update.elapsed().as_secs();

                if (percentage % 5 == 0 && percentage > progress.last_percentage) || elapsed >= 5 {
                    println!("Progress: {}%", percentage);
                    progress.last_update = std::time::Instant::now();
                    progress.last_percentage = percentage;
                }
            }
        }
    }

    if progress.last_percentage < 100 {
        println!("Progress: 100%");
    }

    Ok(())
}

fn make_wallpaper(
    file_path: &str,
    target_dir: &str,
    dest_width: usize,
    dest_height: usize,
    force: bool,
) {
    println!("{}", file_path);

    let output_path = format!(
        "{}/adjusted - {}",
        target_dir,
        Path::new(file_path).file_name().unwrap().to_str().unwrap()
    );

    if Path::new(&output_path).exists() && !force {
        return
    }

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
            .blur(25.0);

        let mut final_image = RgbImage::new(dest_width as u32, dest_height as u32);
        imageops::overlay(&mut final_image, &bg.to_rgb8(), 0, 0);

        let x = ((dest_width as i32) - (resized.width() as i32)) / 2;
        let y = ((dest_height as i32) - (resized.height() as i32)) / 2;
        imageops::overlay(&mut final_image, &resized.to_rgb8(), x as i64, y as i64);

        if let Err(e) = final_image.save(&output_path) {
            eprintln!("Failed to save image {}: {}", output_path, e);
        }
    } else {
        eprintln!("Failed to open image: {}", file_path);
    }
}

fn get_source_dir(args: &Args) -> String {
    let path = args.directory.trim_end_matches('/').to_string();
    shellexpand::tilde(path.as_str()).to_string()
}

fn ensure_target_dir(source: &String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(source);
    let parent = path.parent().ok_or("Source path has no parent directory.")?;
    let dir_name = path.file_name().ok_or("Source path has no file name.")?
        .to_str().ok_or("Directory name contains invalid characters.")?;

    let adjusted_path = parent.join(format!("{} - adjusted", dir_name));

    if !adjusted_path.exists() {
        fs::create_dir_all(&adjusted_path)?;
    }

    Ok(adjusted_path.to_str().unwrap().to_string())
}

fn init_progress(source: &str) -> Result<(usize, Progress), Box<dyn Error>> {
    let entries = fs::read_dir(&source)?;
    let source_items = entries.count();
    println!("Found {} items in source directory", source_items);

    let progress = Progress {
        last_update: std::time::Instant::now(),
        last_percentage: 0,
    };

    Ok((source_items, progress))
}