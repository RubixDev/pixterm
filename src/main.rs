use ansipix::FilterType;
use clap::{Args, Parser};
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct PixTerm {
    /// Paths to directories and files to display
    paths: Vec<PathBuf>,

    #[clap(flatten)]
    config: Config,
}

#[derive(Args, Debug)]
struct Config {
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

fn run(config: &Config, file: &PathBuf) -> Result<(), String> {
    let img = match ansipix::of_image_file_with_filter(
        file.clone(),
        (config.width as usize, config.height as usize),
        config.threshold,
        config.raw,
        if config.aliasing {
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

    if config.filename {
        println!(
            "\x1b[1m{}\x1b[0m",
            file.to_str().unwrap_or("Could not determine file name")
        );
    }
    if !config.silent {
        println!("{}", img);
    }
    if let Some(outfile) = &config.outfile {
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

fn run_all(config: &Config, dir: &PathBuf) -> Result<(), String> {
    for entry in match dir.read_dir() {
        Ok(entries) => entries,
        Err(e) => {
            return Err(format!(
                "\x1b[31mCould not read directory contents of \x1b[1m{}\x1b[22m: {e}",
                dir.to_string_lossy()
            ))
        }
    } {
        match entry {
            Ok(entry) => {
                if let Err(_) = run(config, &entry.path()) {
                    eprintln!(
                        "\x1b[1mSkipping {file}...\x1b[0m\n",
                        file = entry.path().to_string_lossy()
                    );
                }
            }
            Err(e) => {
                eprintln!("\x1b[1mSkipping one file:\x1b[22m {e}\x1b[0m\n");
            }
        }
    }
    Ok(())
}

fn main() {
    let pixterm = PixTerm::parse();
    let paths = &if pixterm.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        pixterm.paths
    };
    for path in paths.iter() {
        if let Err(e) = if path.is_dir() {
            run_all(&pixterm.config, path)
        } else {
            run(&pixterm.config, path)
        } {
            eprintln!("{e}\n");
        }
    }
}
