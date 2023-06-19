mod request;

use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
};

use request::request_n;

pub fn fuzz(
    url: &str,
    source: &str,
    num_parallel: Option<&str>,
    out: Option<&str>,
) -> Result<(), Box<Error>> {
    println!("running fuzz");

    let out = out.unwrap_or_else(|| {
        println!("no output directory specified, writing output to fuzz.txt");
        "fuzz.txt"
    });

    let num_parallel: usize = num_parallel
        .unwrap_or_else(|| {
            println!("using 8 parallel requests");
            "8"
        })
        .parse()?;

    let mut outfile = BufWriter::new(File::create(out)?);
    let sourcefile = BufReader::new(File::open(source)?);
    let mut lines = sourcefile.lines();

    while request_n(&mut lines, num_parallel, url, &mut outfile)? {}

    Ok(())
}
