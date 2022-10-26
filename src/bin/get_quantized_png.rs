use std::fs::File;

use clap::Parser;
use image;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input filepath
    #[arg(short, long)]
    input: String,

    /// Output filepath
    #[arg(short, long)]
    output: String,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_path = File::open(args.input)?;
    let img = image::open(file_path);

    Ok(())
}
