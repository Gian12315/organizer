#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use std::fs;
use structopt::StructOpt;

use glob::glob;

#[derive(StructOpt, Debug)]
#[structopt(name = "organizer")]
struct Opt {
    /// Set directory where we should look for the files
    #[structopt(short = "w", long = "workdir", parse(from_os_str))]
    work_directory: PathBuf,

    /// Set output directory
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output_directory: PathBuf,

    /// Extension of files
    #[structopt(short = "e", long = "ext")]
    file_ext: String,
}

fn main() {
    let mut files = vec![];
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let mut string = String::from(opt.work_directory.as_path().to_str().unwrap());
    string.push_str("\\*");
    string.push_str(&opt.file_ext);
    println!("{}", string);
    match glob(&string) {
        Ok(iter) => {
            iter.for_each(|file| {
                let file = file.unwrap();
                println!("{:?}", file);
                files.push(file);
            })
        }
        Err(err) => println!("Error: {}", err),
    };
    fs::create_dir(&opt.output_directory).unwrap();
    for file in files {
        let string = (*file).file_name().unwrap();
        let mut output_path = opt.output_directory.clone();
        output_path.push(string);
        fs::copy(&*file, output_path).unwrap();
    }
}
