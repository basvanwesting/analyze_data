mod number_stats;
mod output_data;
mod output_row;

use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use number_stats::NumberStats;
use output_data::OutputData;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

/// some number stats on stream (count, min, max, mean, stddev)
#[derive(Parser)]
struct Cli {
    /// input delimiter
    #[arg(short = 'd', long)]
    input_delimiter: char,

    /// Optional output delimiter, default to human readable table output
    #[arg(short = 'D', long)]
    output_delimiter: Option<char>,

    /// Count zeros as null, in addition to always countint non-numbers as null
    #[arg(short, long, default_value_t = false)]
    zero_as_null: bool,

    /// Optional number of decimals to round for output
    #[arg(short = 'r', long, default_value_t = 0)]
    decimals: usize,

    /// The path to the file to read, use - to read from stdin (must not be a tty)
    #[arg(default_value = "-")]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;

    let group_stats = if file == PathBuf::from("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }
        group_stats_in_buf_reader(
            BufReader::new(stdin().lock()),
            args.input_delimiter,
            args.zero_as_null,
        )
    } else {
        group_stats_in_buf_reader(
            BufReader::new(File::open(&file).unwrap()),
            args.input_delimiter,
            args.zero_as_null,
        )
    };
    OutputData::new(
        group_stats,
        args.input_delimiter,
        args.output_delimiter,
        args.decimals,
    )
    .print();
}

fn group_stats_in_buf_reader<R: BufRead>(
    buf_reader: R,
    delimiter: char,
    zero_as_null: bool,
) -> HashMap<String, NumberStats> {
    let mut group_stats = HashMap::<String, NumberStats>::new();
    for line in buf_reader.lines() {
        let raw = line.unwrap();
        match raw.rsplit_once(delimiter) {
            Some((group, value)) => {
                let number_stats = group_stats
                    .entry(group.to_string())
                    .or_insert(NumberStats::new());
                match value.parse::<f64>() {
                    Ok(num) if zero_as_null && num == 0.0 => number_stats.add_null(),
                    Ok(num) => number_stats.add(num),
                    Err(_) => number_stats.add_null(),
                };
            }
            None => {
                group_stats
                    .entry("<INVALID>".to_string())
                    .and_modify(|number_stats| number_stats.add_null())
                    .or_insert(NumberStats::new());
            }
        }
    }
    group_stats
}
