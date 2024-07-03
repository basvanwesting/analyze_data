use crate::number_stats::NumberStats;
use crate::string_stats::StringStats;
use cli_table::{
    format::{HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout, Cell, CellStruct, Style, Table,
};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

type Data = HashMap<String, (StringStats, NumberStats)>;
struct OutputData {
    output_rows: Vec<OutputRow>,
    group_length: usize,
    output_delimiter: Option<char>,
}
pub struct OutputRow {
    pub group_data: Vec<String>,
    pub stats_data: Vec<String>,
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

fn build_data<R: BufRead>(buf_reader: R, delimiter: char, _zero_as_empty: bool) -> Data {
    let mut data = Data::new();
    for line in buf_reader.lines() {
        let raw = line.unwrap();
        match raw.rsplit_once(delimiter) {
            Some((group, value)) => {
                let (value_stats, length_stats) = data
                    .entry(group.to_string())
                    .or_insert((StringStats::new(), NumberStats::new()));

                if value.is_empty() {
                    value_stats.add_empty();
                    length_stats.add_empty();
                } else {
                    value_stats.add(value.to_string());
                    length_stats.add(value.len() as f64);
                };
            }
            None => {
                data.entry("<INVALID>".to_string())
                    .and_modify(|(value_stats, _length_stats)| value_stats.add_error())
                    .or_insert((StringStats::new(), NumberStats::new()));
            }
        }
    }
    data
}

impl OutputData {
    pub fn new(
        data: Data,
        input_delimiter: char,
        output_delimiter: Option<char>,
        precision: usize,
    ) -> Self {
        let output_rows: Vec<OutputRow> = data
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
            .map(|(group, (value_stats, length_stats))| {
                let group_data: Vec<String> = group
                    .split(input_delimiter)
                    .map(|v| v.to_string())
                    .collect();
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
                OutputRow {
                    group_data,
                    stats_data,
                }
            })
            .collect();
        let group_length = output_rows.first().unwrap().group_data.len();
        Self {
            output_rows,
            group_length,
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

        let mut group_title: Vec<CellStruct> = [""]
            .iter()
            .cycle()
            .take(self.group_length)
            .map(|v| v.cell())
            .collect();
        let mut number_title: Vec<CellStruct> = vec![
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
        group_title.append(&mut number_title);

        let table = self
            .output_rows
            .iter()
            .map(|output_row| {
                let mut group_data: Vec<CellStruct> =
                    output_row.group_data.iter().map(|v| v.cell()).collect();
                let mut number_data: Vec<CellStruct> = output_row
                    .stats_data
                    .iter()
                    .map(|v| v.cell().justify(Justify::Right))
                    .collect();
                group_data.append(&mut number_data);
                group_data
            })
            .table()
            .separator(separator)
            .title(group_title)
            .bold(true);

        print_stdout(table).unwrap();
    }

    pub fn print_csv(&self, delimiter: char) {
        let delimiter = delimiter.to_string();
        println!(
            "{}{}",
            delimiter.repeat(self.group_length),
            [
                "count",
                "empty",
                "cardinality",
                "string_min",
                "string_max",
                "length_min",
                "length_max",
                "length_mean",
                "length_stddev"
            ]
            .join(&delimiter)
        );
        for row in self.output_rows.iter() {
            println!(
                "{}{}{}",
                row.group_data.join(&delimiter),
                delimiter,
                row.stats_data.join(&delimiter)
            );
        }
    }
}
