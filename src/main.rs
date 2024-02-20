use std::{fs, path::Path};

use clap::{command, Parser};
use skrifa::FontRef;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Compare these characters to detect duplication
    #[arg(short, long)]
    test_string: String,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for file in args.files {
        let file = Path::new(&file);
        if !file.is_file() {
            eprintln!("{file:?} is not a file");
            continue;
        }
        let raw_font = fs::read(file).unwrap();
        let font = FontRef::new(&raw_font);

        // TODO: render test string => Vec<BezPath> scaled into the same box and see if they are pretty similar
    }
}
