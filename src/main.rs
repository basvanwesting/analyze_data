mod number_stats;
mod output_csv_data;
mod output_number_data;
mod output_row;
mod output_string_data;
mod string_stats;

use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use number_stats::NumberStats;
use output_csv_data::OutputCsvData;
use output_number_data::OutputNumberData;
use output_string_data::OutputStringData;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};
use string_stats::StringStats;

type GroupNumberStats = HashMap<String, NumberStats>;
type GroupStringStats = HashMap<String, (StringStats, NumberStats)>;
type CsvStats = Vec<(String, StringStats, NumberStats, NumberStats)>;

/// Grouped number or string stats on stream (count, min, max, mean, stddev).
/// Takes the last column of the provided data as the number (default) or string value to analyze.
/// All preceding columns are interpreted as grouping data.
#[derive(Parser)]
struct Cli {
    /// input delimiter
    #[arg(short = 'd', long)]
    input_delimiter: char,

    /// Optional output delimiter, default to human readable table output
    #[arg(short = 'D', long)]
    output_delimiter: Option<char>,

    /// Optional number of decimals to round for output
    #[arg(short = 'r', long, default_value_t = 0)]
    decimals: usize,

    /// Count zeros as null in number mode. Count empty strings as null in string mode
    #[arg(short, long, default_value_t = false)]
    null: bool,

    /// Interpret as strings instead of numbers (default), returns stats about length and value
    #[arg(short, long, default_value_t = false)]
    strings: bool,

    /// Interpret the data as CSV, making the headers the groups
    #[arg(short, long, default_value_t = false)]
    csv: bool,

    /// The path to the file to read, use - to read from stdin (must not be a tty)
    #[arg(default_value = "-")]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;

    if args.csv {
        let csv_stats = if file == PathBuf::from("-") {
            if stdin().is_terminal() {
                Cli::command().print_help().unwrap();
                ::std::process::exit(2);
            }
            csv_stats_in_buf_reader(
                BufReader::new(stdin().lock()),
                args.input_delimiter,
                args.null,
            )
        } else {
            csv_stats_in_buf_reader(
                BufReader::new(File::open(&file).unwrap()),
                args.input_delimiter,
                args.null,
            )
        };
        OutputCsvData::new(
            csv_stats,
            args.input_delimiter,
            args.output_delimiter,
            args.decimals,
        )
        .print();
    } else if args.strings {
        let group_string_stats = if file == PathBuf::from("-") {
            if stdin().is_terminal() {
                Cli::command().print_help().unwrap();
                ::std::process::exit(2);
            }
            group_string_stats_in_buf_reader(
                BufReader::new(stdin().lock()),
                args.input_delimiter,
                args.null,
            )
        } else {
            group_string_stats_in_buf_reader(
                BufReader::new(File::open(&file).unwrap()),
                args.input_delimiter,
                args.null,
            )
        };
        OutputStringData::new(
            group_string_stats,
            args.input_delimiter,
            args.output_delimiter,
            args.decimals,
        )
        .print();
    } else {
        let group_number_stats = if file == PathBuf::from("-") {
            if stdin().is_terminal() {
                Cli::command().print_help().unwrap();
                ::std::process::exit(2);
            }
            group_number_stats_in_buf_reader(
                BufReader::new(stdin().lock()),
                args.input_delimiter,
                args.null,
            )
        } else {
            group_number_stats_in_buf_reader(
                BufReader::new(File::open(&file).unwrap()),
                args.input_delimiter,
                args.null,
            )
        };
        OutputNumberData::new(
            group_number_stats,
            args.input_delimiter,
            args.output_delimiter,
            args.decimals,
        )
        .print();
    }
}

fn group_number_stats_in_buf_reader<R: BufRead>(
    buf_reader: R,
    delimiter: char,
    zero_as_null: bool,
) -> GroupNumberStats {
    let mut group_number_stats = GroupNumberStats::new();
    for line in buf_reader.lines() {
        let raw = line.unwrap();
        match raw.rsplit_once(delimiter) {
            Some((group, value)) => {
                let number_stats = group_number_stats
                    .entry(group.to_string())
                    .or_insert(NumberStats::new());
                match value.parse::<f64>() {
                    Ok(num) if zero_as_null && num == 0.0 => number_stats.add_null(),
                    Ok(num) => number_stats.add(num),
                    Err(_) => number_stats.add_null(),
                };
            }
            None => {
                group_number_stats
                    .entry("<INVALID>".to_string())
                    .and_modify(|number_stats| number_stats.add_null())
                    .or_insert(NumberStats::new());
            }
        }
    }
    group_number_stats
}

fn group_string_stats_in_buf_reader<R: BufRead>(
    buf_reader: R,
    delimiter: char,
    empty_as_null: bool,
) -> GroupStringStats {
    let mut group_string_stats = GroupStringStats::new();
    for line in buf_reader.lines() {
        let raw = line.unwrap();
        match raw.rsplit_once(delimiter) {
            Some((group, value)) => {
                let (value_stats, length_stats) = group_string_stats
                    .entry(group.to_string())
                    .or_insert((StringStats::new(), NumberStats::new()));

                if empty_as_null && value.is_empty() {
                    length_stats.add_null();
                    value_stats.add_null();
                } else {
                    length_stats.add(value.len() as f64);
                    value_stats.add(value.to_string());
                };
            }
            None => {
                group_string_stats
                    .entry("<INVALID>".to_string())
                    .and_modify(|(value_stats, _length_stats)| value_stats.add_null())
                    .or_insert((StringStats::new(), NumberStats::new()));
            }
        }
    }
    group_string_stats
}

fn csv_stats_in_buf_reader<R: BufRead>(
    buf_reader: R,
    delimiter: char,
    zero_or_empty_as_null: bool,
) -> CsvStats {
    let mut lines_iter = buf_reader.lines();
    let headers: Vec<String> = lines_iter
        .next()
        .unwrap()
        .expect("at least one row for the header")
        .split(delimiter)
        .map(|v| v.to_string())
        .collect();

    let mut csv_stats: CsvStats = headers
        .into_iter()
        .map(|header| {
            (
                header,
                StringStats::new(),
                NumberStats::new(),
                NumberStats::new(),
            )
        })
        .collect();

    for line in lines_iter {
        for ((_header, string_stats, number_stats, length_stats), value) in
            csv_stats.iter_mut().zip(line.unwrap().split(delimiter))
        {
            if zero_or_empty_as_null && value.is_empty() {
                string_stats.add_null();
                length_stats.add_null();
                number_stats.add_null();
            } else {
                string_stats.add(value.to_string());
                length_stats.add(value.len() as f64);
                match value.parse::<f64>() {
                    Ok(num) if zero_or_empty_as_null && num == 0.0 => number_stats.add_null(),
                    Ok(num) => number_stats.add(num),
                    Err(_) => number_stats.add_null(),
                };
            };
        }
    }
    csv_stats
}
