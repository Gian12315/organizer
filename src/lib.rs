extern crate structopt;

use std::fs;
use std::path::PathBuf;
use std::process;

use glob::glob;
use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(name = "organizer")]
pub struct Config {
    /// Set directory where we should look for the files
    #[structopt(short = "w", long = "workdir", parse(from_os_str))]
    work_directory: PathBuf,

    /// Set output directory
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output_directory: PathBuf,

    /// Extension of the files to copy,
    /// in order to copy all files, without checking the extension
    /// give the argument the value of "" (an empty string)
    #[structopt(short = "e", long = "ext")]
    file_ext: String,
}

pub fn run(config: Config) {
    let pattern = create_glob_pattern(config.work_directory, &config.file_ext);
    let files = find_files(&pattern);
    if files.is_none() {
        eprintln!("Didn't find any files with {} extension.", &config.file_ext);
        process::exit(1);
    }
    if !&config.output_directory.exists() {
        fs::create_dir(&config.output_directory).unwrap();
    }
    move_files(files.unwrap(), config.output_directory);
}

fn create_glob_pattern(path: PathBuf, extension: &str) -> String {
    let mut pattern = String::from((*path).to_str().unwrap());
    pattern.push_str("\\*");
    pattern.push_str(extension);
    pattern
}

fn find_files(pattern: &str) -> Option<Vec<PathBuf>> {
    match glob(pattern) {
        Ok(iter) => Some(
            iter.map(|path| path.unwrap())
                .filter(|path| path.is_file())
                .collect(),
        ),
        Err(err) => {
            eprintln!("Error: {}", err);
            None
        }
    }
}

fn move_files(files: Vec<PathBuf>, path: PathBuf) {
    files.iter().for_each(|file| {
        let string = (*file).file_name().unwrap();
        let mut output_path = path.clone();
        output_path.push(string);
        if !output_path.exists() {
            fs::copy(&*file, output_path).unwrap();
            fs::remove_file(&*file).unwrap();
            println!("Moved: {:?}", string);
        } else {
            println!("File named {:?} already exists.", string);
        }
    });
}
