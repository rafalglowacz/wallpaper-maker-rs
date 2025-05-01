fn main() {
    let args = clap::Command::new("wallpaper-maker")
        .arg(clap::Arg::new("directory").help("Directory containing images").required(true))
        .get_matches();

    let path = args.get_one::<String>("directory").expect("Directory is required");

    let path = path.trim_end_matches('/');
    println!("{}", path);
}
