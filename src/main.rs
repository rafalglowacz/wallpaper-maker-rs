use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    directory: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = get_path(args);

    println!("Reading directory: {}", path);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        println!("{}", entry.path().display());
    }

    Ok(())
}

fn get_path(args: Args) -> String {
    let path = args.directory.trim_end_matches('/').to_string();
    shellexpand::tilde(path.as_str()).to_string()
}
