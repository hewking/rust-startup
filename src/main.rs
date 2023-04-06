use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
    let files = get_files();
    walkdir();
}


use std::fs;

fn get_files() -> std::io::Result<()> {
    let paths = fs::read_dir(".")?;

    for path in paths {
        println!("{}", path?.path().display());
    }

    Ok(())
}

use walkdir::WalkDir;

fn walkdir() {
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        let depth = entry.depth();
        let indent = " ".repeat(depth * 4);
        println!("{}{}", indent, entry.path().display());
    }
}