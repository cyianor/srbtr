use std::fs::File;
use std::io::BufReader;

mod transcoder;
use transcoder::Transcoder;

fn main() -> Result<(), std::io::Error> {
    let f = File::open("test.txt")?;
    let input = BufReader::new(f);

    let mut input_str = String::new();
    let mut output_str = String::new();

    let tr = Transcoder::from(input);

    for ch in tr {
        match ch {
            Ok(v) => {
                input_str.push_str(&v.0);
                output_str.push_str(&v.1);
            }
            Err(err) => return Err(err),
        }
    }

    println!("{}", input_str);
    println!("{}", output_str);

    Ok(())
}
