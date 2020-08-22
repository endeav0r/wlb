//! Command-line invocation of wlb scripts. Useful for testing.

use std::error::Error;
use std::fs::File;
use std::io::Read;

fn run_file<S: AsRef<str>>(filename: S) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(filename.as_ref())?;

    let mut script: String = String::new();
    file.read_to_string(&mut script)?;

    let mut context = wlb::Context::new()?;
    context.execute(&script)?;

    Ok(())
}

fn main() {
    let matches = clap::App::new("wlb-test")
        .arg(
            clap::Arg::with_name("script-path")
                .short("s")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if let Some(error) = run_file(matches.value_of("script-path").unwrap()).err() {
        println!("{}", error);
    }
}
