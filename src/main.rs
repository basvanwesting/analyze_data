mod number_stats;

use clap::{CommandFactory, Parser};
use cli_table::{
    format::{HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout, Cell, CellStruct, Style, Table,
};
use is_terminal::IsTerminal as _;
use number_stats::NumberStats;
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
    #[arg(short, long)]
    delimiter: char,

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
            args.delimiter,
            args.zero_as_null,
        )
    } else {
        group_stats_in_buf_reader(
            BufReader::new(File::open(&file).unwrap()),
            args.delimiter,
            args.zero_as_null,
        )
    };
    let formatted_data = group_stats_formatted_data(group_stats, args.decimals);
    match args.output_delimiter {
        None => print_group_stats_table(formatted_data),
        Some(delimiter) => print_group_stats_csv(formatted_data, delimiter),
    }
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

fn group_stats_formatted_data(
    group_stats: HashMap<String, NumberStats>,
    decimals: usize,
) -> Vec<Vec<String>> {
    let mut group_stats_vec: Vec<_> = group_stats.into_iter().collect();
    group_stats_vec.sort_by(|a, b| a.0.cmp(&b.0));

    let mut data = vec![];
    for (group, number_stats) in group_stats_vec {
        data.push(vec![
            group,
            format!("{}", number_stats.count()),
            format!("{}", number_stats.null_count()),
            format!("{:.*}", decimals, number_stats.min().unwrap_or(0.0),),
            format!("{:.*}", decimals, number_stats.max().unwrap_or(0.0),),
            format!("{:.*}", decimals, number_stats.mean()),
            format!("{:.*}", decimals, number_stats.stddev()),
        ]);
    }
    data
}

fn print_group_stats_table(formatted_data: Vec<Vec<String>>) {
    let separator = Separator::builder()
        .title(Some(HorizontalLine::default()))
        .column(Some(VerticalLine::default()))
        .build();
    let table = formatted_data
        .table()
        .separator(separator)
        .title(vec![
            "Group".cell().bold(true),
            "Count".cell().bold(true),
            "NULL".cell().bold(true),
            "Min".cell().bold(true),
            "Max".cell().bold(true),
            "Mean".cell().bold(true),
            "StdDev".cell().bold(true),
        ])
        .bold(true);

    print_stdout(table).unwrap();
}

fn print_group_stats_csv(formatted_data: Vec<Vec<String>>, delimiter: char) {
    let delimiter = delimiter.to_string();
    println!(
        "{}",
        ["group", "count", "null", "min", "max", "mean", "stddev"].join(&delimiter)
    );
    for row in formatted_data {
        println!("{}", row.join(&delimiter));
    }
}
