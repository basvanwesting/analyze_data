mod number_stats;

use clap::{CommandFactory, Parser};
use is_terminal::IsTerminal as _;
use number_stats::NumberStats;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::PathBuf,
};

/// some number stats on stream (count, min, max, mean, stddev)
#[derive(Parser)]
struct Cli {
    /// Count zeros as null, in addition to always countint non-numbers as null
    #[arg(short, long, default_value_t = false)]
    zero_as_null: bool,

    /// Optional number of decimals for output
    #[arg(short, long, default_value_t = 0)]
    decimals: usize,

    /// The path to the file to read, use - to read from stdin (must not be a tty)
    #[arg(default_value = "-")]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;

    let number_stats = if file == PathBuf::from("-") {
        if stdin().is_terminal() {
            Cli::command().print_help().unwrap();
            ::std::process::exit(2);
        }

        number_stats_in_buf_reader(BufReader::new(stdin().lock()), args.zero_as_null)
    } else {
        number_stats_in_buf_reader(
            BufReader::new(File::open(&file).unwrap()),
            args.zero_as_null,
        )
    };

    println!("Count:        {}", number_stats.count());
    println!("Count (NULL): {}", number_stats.null_count());
    println!(
        "Min:          {:.*}",
        args.decimals,
        number_stats.min().unwrap_or(0.0)
    );
    println!(
        "Max:          {:.*}",
        args.decimals,
        number_stats.max().unwrap_or(0.0)
    );
    println!("Mean:         {:.*}", args.decimals, number_stats.mean());
    println!("StdDev:       {:.*}", args.decimals, number_stats.stddev());
}

fn number_stats_in_buf_reader<R: BufRead>(buf_reader: R, zero_as_null: bool) -> NumberStats {
    let mut number_stats = NumberStats::new();
    for line in buf_reader.lines() {
        let word = line.unwrap();
        match word.parse::<f64>() {
            Ok(num) if zero_as_null && num == 0.0 => number_stats.add_null(),
            Ok(num) => number_stats.add(num),
            Err(_) => number_stats.add_null(),
        };
    }
    number_stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use std::io::Cursor;

    #[test]
    fn test_number_stats_in_buf_reader() {
        let data = "1\n2\n3\nfoo\n4\n5\n";
        let cursor = Cursor::new(data);
        let stats = number_stats_in_buf_reader(cursor, false);

        assert_eq!(stats.count(), 5);
        assert_eq!(stats.null_count(), 1);
        assert_eq!(stats.mean(), 3.0);
        assert_abs_diff_eq!(stats.stddev(), 1.41, epsilon = 0.01);
        assert_eq!(stats.min(), Some(1.0));
        assert_eq!(stats.max(), Some(5.0));
    }
}
