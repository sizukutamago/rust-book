use clap::Parser;
use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    input: PathBuf,
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    create_dir_all(&args.output).unwrap();

    let mut processed_count = 0;
    for item in read_dir(&args.input).unwrap() {
        let item = item.unwrap();
        let input_path = item.path();

        if input_path.is_dir() {
            continue;
        }

        let img = image::open(&input_path);
        if let Ok(img) = img {
            let thumbnail = img.thumbnail(64, 64);
            let output_path = args.output.join(input_path.file_name().unwrap());

            thumbnail.save(output_path).unwrap();
            processed_count += 1;
        }
    }

    println!("Processed {} images", processed_count);
}
