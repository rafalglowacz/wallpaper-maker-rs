use clap::Parser;
use std::fs;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    directory: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = get_path(args);

    println!("Reading directory: {}", path);

    let image_pattern = Regex::new(r"\.(jpe?g|png|webp)$").unwrap();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            if image_pattern.is_match(path_str) {
                make_wallpaper(path_str);
            }
        }
    }

    Ok(())
}

fn make_wallpaper(file_path: &str) {
    println!("{}", file_path);
}

fn get_path(args: Args) -> String {
    let path = args.directory.trim_end_matches('/').to_string();
    shellexpand::tilde(path.as_str()).to_string()
}
