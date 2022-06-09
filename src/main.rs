use std::{fs::File, io::Write, path::PathBuf};
use structopt::StructOpt;

/// A CLI tool to show images in a terminal
#[derive(StructOpt, Debug)]
#[structopt(author)]
struct PixTerm {
    /// Path to image file(s) to display
    #[structopt(required = true)]
    files: Vec<PathBuf>,

    /// Maximum width in pixels of the resized image. Also see --height
    #[structopt(short = "W", long, default_value = "32")]
    width: u16,

    /// Maximum height in pixels of the resized image. Also see --width
    #[structopt(short = "H", long, default_value = "32")]
    height: u16,

    /// Minimum alpha value of a pixel for it to be shown
    #[structopt(short, long, default_value = "50")]
    threshold: u8,

    /// File to write the resulting string into. See --raw to get literal escape sequences and --silent to suppress stdout
    #[structopt(short, long)]
    outfile: Option<PathBuf>,

    /// Print escape sequences literal
    #[structopt(short, long)]
    raw: bool,

    /// Do not print to stdout. Useful with --outfile
    #[structopt(short, long)]
    silent: bool,
}

fn run(pixterm: &PixTerm, file: &PathBuf) -> Result<(), String> {
    let img = match ansipix::of_image(
        file.clone(),
        (pixterm.width as usize, pixterm.height as usize),
        pixterm.threshold,
        pixterm.raw
    ) {
        Ok(img) => img,
        Err(_) => return Err(format!(
            "\x1b[1;31m{}\x1b[22m could not be opened as an image. Does it exist? Is it an image?\x1b[0m",
            file.to_str().unwrap_or("The specified file")
        )),
    };

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
    let pixterm = PixTerm::from_args();
    for file in pixterm.files.iter() {
        match run(&pixterm, file) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e),
        };
    }
}
