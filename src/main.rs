use std::{path::PathBuf, process};
use image::{GenericImageView, imageops::FilterType};
use structopt::StructOpt;

const TOP_HALF:    &str = "\u{2580}";
const BOTTOM_HALF: &str = "\u{2584}";

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

    let img = image::open(&pixterm.file).unwrap_or_else(|_| {
        println!(
            "\x1b[1;31m{}\x1b[22m could not be opened as an image. Does it exist? Is it an image?\x1b[0m",
            &pixterm.file.to_str().unwrap_or("The specified file")
        );
        process::exit(1);
    });

    let mut pixels: Vec<Vec<[u8; 4]>> = vec![];

    for (x, y, pix) in img.resize(pixterm.width as u32, pixterm.height as u32, FilterType::Nearest).pixels() {
        if x == 0 { pixels.push(vec![]); }
        pixels[y as usize].push(pix.0);
    }

    for line in (0..pixels.len()).filter(|index| index % 2 == 0) {
        for char in 0..pixels[line].len() {
            let top_pix: [u8; 4] = pixels[line][char];
            let bot_pix: [u8; 4] = if line + 1 >= pixels.len() { [0; 4] } else { pixels[line + 1][char] };
            let top_invis: bool = top_pix[3] < pixterm.threshold;
            let bot_invis: bool = bot_pix[3] < pixterm.threshold;

            if top_invis && bot_invis {
                print!(" ");
            } else if top_invis && !bot_invis {
                print!("\x1b[38;2;{};{};{}m{}\x1b[0m", bot_pix[0], bot_pix[1], bot_pix[2], BOTTOM_HALF);
            } else if !top_invis && bot_invis {
                print!("\x1b[38;2;{};{};{}m{}\x1b[0m", top_pix[0], top_pix[1], top_pix[2], TOP_HALF);
            } else {
                print!(
                    "\x1b[38;2;{};{};{};48;2;{};{};{}m{}\x1b[0m",
                    bot_pix[0], bot_pix[1], bot_pix[2],
                    top_pix[0], top_pix[1], top_pix[2],
                    BOTTOM_HALF
                );
            }
        }
        println!()
    }
}
