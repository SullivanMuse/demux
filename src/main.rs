use clap::Parser;
use regex::Regex;
use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    groups: Vec<String>,

    #[arg(short, long, value_name = "input-file")]
    input_path: Option<PathBuf>,

    #[arg(short, long, value_name = "output-file")]
    output_path: Option<PathBuf>,
}

fn input_lines(cli: &Cli) -> Box<dyn Iterator<Item = String>> {
    if let Some(input_path) = &cli.input_path {
        let input_file = File::open(input_path).unwrap();
        Box::new(BufReader::new(input_file).lines().map(|x| x.unwrap()))
    } else {
        Box::new(stdin().lines().map(|x| x.unwrap()))
    }
}

fn output_writer(cli: &Cli) -> Box<dyn Write> {
    if let Some(output_path) = &cli.output_path {
        let output_file = File::open(output_path).unwrap();
        Box::new(BufWriter::new(output_file))
    } else {
        Box::new(BufWriter::new(stdout()))
    }
}

fn main() {
    let cli = Cli::parse();
    let group_patterns = cli
        .groups
        .iter()
        .map(|group| Regex::new(&group).unwrap())
        .collect::<Vec<_>>();
    let lines = input_lines(&cli);
    let mut output = output_writer(&cli);

    // Indent each line appropriately and output
    for line in lines {
        for (i, group) in group_patterns.iter().enumerate() {
            if group.is_match(&line) {
                for _ in 0..i {
                    write!(&mut output, "        ").unwrap();
                }
                writeln!(&mut output, "[{}] {}", &cli.groups[i], line).unwrap();
            }
        }
    }
}
