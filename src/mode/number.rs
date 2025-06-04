use crate::number_stats::NumberStats;
use cli_table::{
    format::{HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout, Cell, CellStruct, Style, Table,
};
use std::io::BufRead;

type Data = NumberStats;
struct OutputData {
    stats_data: Vec<String>,
    output_delimiter: Option<char>,
}

pub fn run<R: BufRead>(
    buf_reader: R,
    input_delimiter: char,
    output_delimiter: Option<char>,
    precision: usize,
    zero_as_empty: bool,
) {
    let data = build_data(buf_reader, input_delimiter, zero_as_empty);
    OutputData::new(data, input_delimiter, output_delimiter, precision).print();
}

fn build_data<R: BufRead>(buf_reader: R, _delimiter: char, zero_as_empty: bool) -> Data {
    let mut number_stats = Data::new();
    for line in buf_reader.lines() {
        let value = line.unwrap();
        if value.is_empty() {
            number_stats.add_empty();
        } else {
            match value.parse::<f64>() {
                Ok(num) if zero_as_empty && num == 0.0 => number_stats.add_empty(),
                Ok(num) => number_stats.add(num),
                Err(_) => number_stats.add_error(),
            };
        }
    }
    number_stats
}

impl OutputData {
    pub fn new(
        number_stats: Data,
        _input_delimiter: char,
        output_delimiter: Option<char>,
        precision: usize,
    ) -> Self {
        let stats_data = vec![
            format!("{}", number_stats.count()),
            format!("{}", number_stats.empty_count()),
            format!("{}", number_stats.error_count()),
            format!("{:.*}", precision, number_stats.min().unwrap_or(0.0),),
            format!("{:.*}", precision, number_stats.max().unwrap_or(0.0),),
            format!("{:.e}", number_stats.sum()),
            format!("{:.*}", precision, number_stats.mean()),
            format!("{:.*}", precision, number_stats.stddev()),
        ];
        Self {
            stats_data,
            output_delimiter,
        }
    }

    pub fn print(&self) {
        match self.output_delimiter {
            None => self.print_table(),
            Some(delimiter) => self.print_csv(delimiter),
        }
    }

    pub fn print_table(&self) {
        let separator = Separator::builder()
            .title(Some(HorizontalLine::default()))
            .column(Some(VerticalLine::default()))
            .build();

        let stats_title: Vec<CellStruct> = vec![
            "Count".cell().justify(Justify::Right).bold(true),
            "Empty".cell().justify(Justify::Right).bold(true),
            "Error".cell().justify(Justify::Right).bold(true),
            "Min".cell().justify(Justify::Right).bold(true),
            "Max".cell().justify(Justify::Right).bold(true),
            "Sum".cell().justify(Justify::Right).bold(true),
            "Mean".cell().justify(Justify::Right).bold(true),
            "StdDev".cell().justify(Justify::Right).bold(true),
        ];

        let table = stats_title
            .into_iter()
            .zip(&self.stats_data)
            .map(|(title, data)| vec![title, data.cell().justify(Justify::Right)])
            .collect::<Vec<Vec<CellStruct>>>()
            .table()
            .separator(separator);

        print_stdout(table).unwrap();
    }

    pub fn print_csv(&self, delimiter: char) {
        let delimiter = delimiter.to_string();
        let stats_title = [
            "count", "empty", "error", "min", "max", "sum", "mean", "stddev",
        ];
        stats_title
            .iter()
            .zip(&self.stats_data)
            .for_each(|(title, data)| {
                println!("{}{}{}", title, delimiter, data,);
            });
    }
}
