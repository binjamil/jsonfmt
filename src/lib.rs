use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn get_args() -> Result<String, Box<dyn Error>> {
    let matches = App::new("jsonfmt")
        .version("0.1.0")
        .author("Muhammad Bin Jamil <binjamil35@gmail.com>")
        .about("jsonfmt - JSON pretty print utility")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file")
                .default_value("-"),
        )
        .get_matches();

    Ok(matches.value_of("file").unwrap().to_string())
}

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let fh = open(filename)?;
    let mut indent = 0;

    for line in fh.lines() {
        let line = line?;
        for c in line.chars() {
            match c {
                '{' => {
                    indent += 2;
                    print!("{}\n{}", c, " ".repeat(indent));
                }
                '}' => {
                    indent -= 2;
                    print!("\n{}{}", " ".repeat(indent), c);
                }
                ' ' => continue,
                ':' => print!("{} ", c),
                ',' => print!("{}\n{}", c, " ".repeat(indent)),
                _ => print!("{}", c),
            }
        }
    }
    println!();

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
