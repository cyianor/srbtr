use clap::Parser;
use std::fs::File;
use std::io::BufReader;

use srbtr::transcoder::Transcoder;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "Tool for transliterating Serbian latin to Serbian Cyrillic text"
)]
struct Args {
    /// Path to source file
    path: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let f = File::open(args.path.expect("expected file path"))?;
    let input = BufReader::new(f);

    let (input_str, output_str): (String, String) =
        Transcoder::from(input).map(|c| c.unwrap()).unzip();

    println!("{}", input_str);
    println!("{}", output_str);

    Ok(())
}
