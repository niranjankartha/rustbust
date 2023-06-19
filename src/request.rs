use reqwest::Client;
use std::{
    error::Error,
    io::{stdout, BufRead, BufWriter, Lines, Write},
    sync::{mpsc::channel, Arc},
    thread::{sleep, spawn},
    time::Duration,
};

pub struct Response {
    pub name: String,
    pub status: u16,
}

pub fn request_n<T, U>(
    lines: &mut Lines<T>,
    n: usize,
    url: &str,
    outfile: &mut BufWriter<U>,
) -> Result<bool, Box<Error>>
where
    T: BufRead,
    U: Write,
{
    let cli = Arc::new(Client::new());
    let (tx, rx) = channel();
    let lines = lines.take(n).collect::<Result<Vec<String>, _>>()?;
    let len = lines.len();

    print!("{}[2K", 27 as char);
    print!("fuzzing {}..{}\r", &lines[0], &lines[len - 1]);
    stdout().flush()?;

    for line in lines {
        let fuzz_url = format!("{}/{}", url, &line);
        let cli = Arc::clone(&cli);
        let tx = tx.clone();

        spawn(move || {
            let mut counter = 0;

            loop {
                match cli.get(&fuzz_url).send() {
                    Ok(res) => {
                        let s = res.status().as_u16();

                        if s != 404 {
                            println!("\n{} returned {}", &line, s);

                            tx.send(Some(Response {
                                status: s,
                                name: line,
                            }))
                            .unwrap();
                        } else {
                            tx.send(None).unwrap();
                        }

                        break;
                    }

                    Err(e) => {
                        counter += 1;

                        println!(
                            "\nerror encountered while trying to fuzz {}: {}\nretrying in 1s",
                            &line, e
                        );

                        if counter > 5 {
                            println!(
                                "\ntoo many errors encountered trying to fuzz {}, skipped",
                                line
                            );

                            tx.send(Some(Response {
                                name: line,
                                status: 500,
                            }))
                            .unwrap();

                            break;
                        }

                        sleep(Duration::from_secs(1));
                    }
                }
            }
        });
    }

    for _ in 0..len {
        if let Some(res) = rx.recv()? {
            outfile.write(&format!("{}\t{}\n", res.name, res.status).into_bytes())?;
        }
    }

    outfile.flush()?;

    Ok(n == len)
}
