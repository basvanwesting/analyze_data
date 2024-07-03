use crate::number_stats::NumberStats;
use crate::string_stats::StringStats;
use cli_table::{
    format::{HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout, Cell, CellStruct, Style, Table,
};
use std::io::BufRead;

type Data = (StringStats, NumberStats);
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

fn build_data<R: BufRead>(buf_reader: R, _delimiter: char, _zero_as_empty: bool) -> Data {
    let (mut value_stats, mut length_stats) = (StringStats::new(), NumberStats::new());
    for line in buf_reader.lines() {
        let value = line.unwrap();
        if value.is_empty() {
            value_stats.add_empty();
            length_stats.add_empty();
        } else {
            value_stats.add(value.to_string());
            length_stats.add(value.len() as f64);
        };
    }
    (value_stats, length_stats)
}

impl OutputData {
    pub fn new(
        string_stats: Data,
        _input_delimiter: char,
        output_delimiter: Option<char>,
        precision: usize,
    ) -> Self {
        let (value_stats, length_stats) = string_stats;
        let stats_data = vec![
            format!("{}", value_stats.count()),
            format!("{}", value_stats.empty_count()),
            format!("{}", value_stats.cardinality()),
            format!("{}", value_stats.min().unwrap_or("".to_string())),
            format!("{}", value_stats.max().unwrap_or("".to_string())),
            format!("{:.*}", 0, length_stats.min().unwrap_or(0.0),),
            format!("{:.*}", 0, length_stats.max().unwrap_or(0.0),),
            format!("{:.*}", precision, length_stats.mean()),
            format!("{:.*}", precision, length_stats.stddev()),
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
            "Cardinality".cell().justify(Justify::Right).bold(true),
            "String Min".cell().justify(Justify::Right).bold(true),
            "String Max".cell().justify(Justify::Right).bold(true),
            "Length Min".cell().justify(Justify::Right).bold(true),
            "Length Max".cell().justify(Justify::Right).bold(true),
            "Length Mean".cell().justify(Justify::Right).bold(true),
            "Length StdDev".cell().justify(Justify::Right).bold(true),
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
            "count",
            "empty",
            "cardinality",
            "string_min",
            "string_max",
            "length_min",
            "length_max",
            "length_mean",
            "length_stddev",
        ];
        stats_title
            .iter()
            .zip(&self.stats_data)
            .for_each(|(title, data)| {
                println!("{}{}{}", title, delimiter, data,);
            });
    }
}
