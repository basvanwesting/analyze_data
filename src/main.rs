mod mode;
mod number_stats;
mod string_stats;

use clap::{CommandFactory, Parser, ValueEnum};
use is_terminal::IsTerminal as _;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Run stats on input as number
    Number,
    /// Run stats on last column as number and interpret preceding columns as group
    GroupNumber,
    /// Run stats on last column as string and interpret preceding columns as group
    GroupString,
    /// Interpret input as CSV with headers and run stats for all
    Csv,
}

/// Analyze data from stream or file
/// Preferable chain with sanitize_csv for input conditioning
/// TODO: handle escape characters
#[derive(Parser)]
struct Cli {
    /// input delimiter
    #[arg(short = 'd', long, default_value_t = ',')]
    input_delimiter: char,

    /// Optional output delimiter, default to human readable table output
    #[arg(short = 'D', long)]
    output_delimiter: Option<char>,

    /// Optional number of decimals to round for output
    #[arg(short, long, default_value_t = 0)]
    precision: usize,

    /// Count zeros as empty when parsing numbers
    #[arg(short, long, default_value_t = false)]
    zero_as_empty: bool,

    /// What mode to run the program in
    #[arg(value_enum, default_value = "number")]
    mode: Mode,

    /// The path to the file to read, use - to read from stdin (must not be a tty)
    #[arg(default_value = "-")]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;

    let buf_reader: Box<dyn BufRead> = if file == PathBuf::from("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }
        Box::new(BufReader::new(stdin().lock()))
    } else {
        Box::new(BufReader::new(File::open(&file).unwrap()))
    };

    match args.mode {
        Mode::Csv => mode::csv::run(
            buf_reader,
            args.input_delimiter,
            args.output_delimiter,
            args.precision,
            args.zero_as_empty,
        ),

        Mode::GroupString => mode::group_string::run(
            buf_reader,
            args.input_delimiter,
            args.output_delimiter,
            args.precision,
            args.zero_as_empty,
        ),

        Mode::GroupNumber => mode::group_number::run(
            buf_reader,
            args.input_delimiter,
            args.output_delimiter,
            args.precision,
            args.zero_as_empty,
        ),
        Mode::Number => mode::number::run(
            buf_reader,
            args.input_delimiter,
            args.output_delimiter,
            args.precision,
            args.zero_as_empty,
        ),
    }
}
