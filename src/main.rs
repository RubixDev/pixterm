use std::{fs::File, io::Write, path::PathBuf};
use ansipix::FilterType;
use clap::Parser;

/// A CLI tool to show images in a terminal
#[derive(Parser, Debug)]
#[clap(author)]
struct PixTerm {
    /// Path to image file(s) to display
    #[clap(required = true)]
    files: Vec<PathBuf>,

    /// Maximum width in pixels of the resized image. Also see --height
    #[clap(short = 'W', long, default_value = "32")]
    width: u16,

    /// Maximum height in pixels of the resized image. Also see --width
    #[clap(short = 'H', long, default_value = "32")]
    height: u16,

    /// Minimum alpha value of a pixel for it to be shown
    #[clap(short, long, default_value = "50")]
    threshold: u8,

    /// File to write the resulting string into. See --raw to get literal escape sequences and --silent to suppress stdout
    #[clap(short, long)]
    outfile: Option<PathBuf>,

    /// Print escape sequences literal
    #[clap(short, long)]
    raw: bool,

    /// Do not print to stdout. Useful with --outfile
    #[clap(short, long)]
    silent: bool,

    /// Print the filename above each picture
    #[clap(short, long)]
    filename: bool,

    /// Use CatmullRom (cubic) iterpolation while resizing
    #[clap(short, long)]
    aliasing: bool,
}

fn run(pixterm: &PixTerm, file: &PathBuf) -> Result<(), String> {
    let img = match ansipix::of_image_file_with_filter(
        file.clone(),
        (pixterm.width as usize, pixterm.height as usize),
        pixterm.threshold,
        pixterm.raw,
        if pixterm.aliasing {
            FilterType::CatmullRom
        } else {
            FilterType::Nearest
        },
    ) {
        Ok(img) => img,
        Err(_) => return Err(format!(
            "\x1b[1;31m{}\x1b[22m could not be opened as an image. Does it exist? Is it an image?\x1b[0m",
            file.to_str().unwrap_or("The specified file")
        )),
    };

    if pixterm.filename {
        println!("\x1b[1m{}\x1b[0m", file.to_str().unwrap_or("Could not determine file name"));
    }
    if !pixterm.silent {
        println!("{}", img);
    }
    if let Some(outfile) = &pixterm.outfile {
        if outfile.exists() {
            return Err(format!(
                "\x1b[1;31m{}\x1b[22m already exists.\x1b[0m",
                outfile.to_str().unwrap_or("The specified output file")
            ));
        }
        let mut file = match File::create(outfile) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!(
                    "\x1b[1;31mError while creating the file:\x1b[22m {}\x1b[0m",
                    e
                ))
            }
        };
        match file.write_all(img.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!(
                    "\x1b[1;31mError while writing to the file:\x1b[22m {}\x1b[0m",
                    e
                ))
            }
        }
    }
    Ok(())
}

fn main() {
    let pixterm = PixTerm::parse();
    for file in pixterm.files.iter() {
        match run(&pixterm, file) {
            Ok(_) => {}
            Err(e) => eprintln!("{}\n", e),
        };
    }
}
