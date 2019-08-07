extern crate structopt;

use std::fs;
use std::path::PathBuf;

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

    /// Extension of the files to copy
    #[structopt(short = "e", long = "ext")]
    file_ext: String,
}

pub fn run(config: Config) {
    let pattern = create_glob_pattern(config.work_directory, &config.file_ext);
    let files = find_files(&pattern);
    if files.is_empty() {
        println!("Didn't find any files with {} extension.", &config.file_ext);
    } else {
        if !&config.output_directory.exists() {
            fs::create_dir(&config.output_directory).unwrap();
        }
        copy_files(files, config.output_directory);
    }
}

fn create_glob_pattern(path: PathBuf, extension: &str) -> String {
    let mut pattern = String::from((*path).to_str().unwrap());
    pattern.push_str("\\*");
    pattern.push_str(extension);
    pattern
}

fn find_files(pattern: &str) -> Vec<PathBuf> {
    let mut files = vec![];
    match glob(pattern) {
        Ok(iter) => iter.for_each(|file| {
            let file = file.unwrap();
            files.push(file);
        }),
        Err(err) => println!("Error: {}", err),
    };
    files
}

fn copy_files(files: Vec<PathBuf>, path: PathBuf) {
    files.iter().for_each(|file| {
        let string = (*file).file_name().unwrap();
        let mut output_path = path.clone();
        output_path.push(string);
        if !output_path.exists() {
            fs::copy(&*file, output_path).unwrap();
            println!("Copied: {:?}", string);
        } else {
            println!("File named {:?} already exists.", string);
        }
    });
}
