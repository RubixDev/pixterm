use std::{fs::File, io::Write, path::PathBuf, process};
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

fn run(pixterm: &PixTerm, file: &PathBuf) {
    let img = ansipix::of_image(
        file.clone(),
        (pixterm.width as usize, pixterm.height as usize),
        pixterm.threshold,
        pixterm.raw
    ).unwrap_or_else(|_| {
        eprintln!(
            "\x1b[1;31m{}\x1b[22m could not be opened as an image. Does it exist? Is it an image?\x1b[0m",
            file.to_str().unwrap_or("The specified file")
        );
        process::exit(1);
    });

    if !pixterm.silent {
        println!("{}", img);
    }
    if pixterm.outfile.is_some() {
        let outfile = pixterm.outfile.clone().unwrap();
        if outfile.exists() {
            eprintln!(
                "\x1b[1;31m{}\x1b[22m already exists.\x1b[0m",
                outfile.to_str().unwrap_or("The specified output file")
            );
            process::exit(2);
        }
        let mut file = File::create(outfile).unwrap_or_else(|e| {
            eprintln!("\x1b[1;31mError while creating the file:\x1b[22m {}", e);
            process::exit(3);
        });
        file.write_all(img.as_bytes()).unwrap_or_else(|e| {
            eprintln!("\x1b[1;31mError while writing to the file:\x1b[22m {}", e);
            process::exit(4);
        });
    }
}

fn main() {
    let pixterm = PixTerm::from_args();
    for file in pixterm.files.iter() {
        run(&pixterm, file);
    }
}
