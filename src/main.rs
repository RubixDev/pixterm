use std::{path::PathBuf, process};
use structopt::StructOpt;

/// A CLI tool to show images in a terminal
#[derive(StructOpt, Debug)]
#[structopt(author)]
struct PixTerm {
    /// Path to image file to display
    #[structopt(short, long)]
    file: PathBuf,

    /// Maximum width in pixels of the resized image
    #[structopt(short = "W", long, default_value = "32")]
    width: u16,

    /// Maximum height in pixels of the resized image
    #[structopt(short = "H", long, default_value = "32")]
    height: u16,

    /// Minimum alpha value of a pixel for it to be shown
    #[structopt(short, long, default_value = "50")]
    threshold: u8,
}

fn main() {
    let pixterm = PixTerm::from_args();

    let img = ansipix::of_image(
        pixterm.file.clone(),
        (pixterm.width as usize, pixterm.height as usize),
        pixterm.threshold
    ).unwrap_or_else(|_| {
        println!(
            "\x1b[1;31m{}\x1b[22m could not be opened as an image. Does it exist? Is it an image?\x1b[0m",
            &pixterm.file.to_str().unwrap_or("The specified file")
        );
        process::exit(1);
    });
    println!("{}", img);
}
