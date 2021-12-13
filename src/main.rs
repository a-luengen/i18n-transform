use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "i18n-transform",
    about = "CLI Tool to convert a i18n files and certain related directory structures into single csv files and vice-versa."
)]
struct Cli {
    #[structopt(parse(from_os_str), short = "t", long = "to")]
    to_file_or_dir: Option<std::path::PathBuf>,
    #[structopt(
        parse(from_os_str),
        short = "o",
        long = "output",
        default_value = "./output.csv"
    )]
    csv_filepath: std::path::PathBuf,
    #[structopt(parse(from_os_str), short = "f", long = "from")]
    from_file_or_dir: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    if let Some(val) = args.to_file_or_dir {
        println!("{}", val.into_os_string().into_string().unwrap())
    }
    println!(
        "{}",
        args.csv_filepath.into_os_string().into_string().unwrap()
    );
    if let Some(val) = args.from_file_or_dir {
        println!("{}", val.into_os_string().into_string().unwrap())
    }

    // determine which mode to run
}
