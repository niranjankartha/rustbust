use clap::clap_app;
use rustbust::fuzz;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let matches = clap_app!(fuzz =>
        (about: "Fuzzes a webpage, but in a fail-safe and parallel way")
        (version: "0.1.0")
        (@arg URL: +required "URL to fuzz")
        (@arg source: -s --source +takes_value +required "Wordlist (file/directory) to use. Uses ./common.txt by default")
        (@arg parallel_count: -p --parallel_count +takes_value "Set number of parallel requests")
        (@arg outfile: -o --outfile +takes_value "Set the file to write output to")
    )
    .get_matches();

    let url = matches.value_of("URL").unwrap();
    let url = if url.chars().nth(url.len() - 1) == Some('/') {
        &url[0..url.len() - 1]
    } else {
        url
    };

    fuzz(
        url,
        matches.value_of("source").unwrap(),
        matches.value_of("parallel_count"),
        matches.value_of("outfile"),
    )?;

    Ok(())
}
