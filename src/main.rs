use anyhow::{Result};
use clap::Parser;

use foireann::teamsheet_generator::generate_teamsheets;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "files/members.csv")]
    input_path: String,
    #[clap(short, long, default_value = "files/male_members.csv")]
    male_output_path: String,
    #[clap(short, long, default_value = "files/female_members.csv")]
    female_output_path: String
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    let file = args.input_path;
    let male_file = args.male_output_path;
    let female_file = args.female_output_path;

    generate_teamsheets(&file, &male_file, &female_file)
}
