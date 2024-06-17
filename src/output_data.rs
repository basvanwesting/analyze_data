use crate::number_stats::NumberStats;
use crate::output_row::OutputRow;
use cli_table::{
    format::{HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout, Cell, CellStruct, Style, Table,
};
use itertools::Itertools;
use std::collections::HashMap;

pub struct OutputData {
    output_rows: Vec<OutputRow>,
    group_length: usize,
    output_delimiter: Option<char>,
}

impl OutputData {
    pub fn new(
        group_stats: HashMap<String, NumberStats>,
        input_delimiter: char,
        output_delimiter: Option<char>,
        decimals: usize,
    ) -> Self {
        let output_rows: Vec<OutputRow> = group_stats
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
            .map(|(group, number_stats)| {
                let group_data: Vec<String> = group
                    .split(input_delimiter)
                    .map(|v| v.to_string())
                    .collect();
                let number_data = vec![
                    format!("{}", number_stats.count()),
                    format!("{}", number_stats.null_count()),
                    format!("{:.*}", decimals, number_stats.min().unwrap_or(0.0),),
                    format!("{:.*}", decimals, number_stats.max().unwrap_or(0.0),),
                    format!("{:.*}", decimals, number_stats.mean()),
                    format!("{:.*}", decimals, number_stats.stddev()),
                ];
                OutputRow::new(group_data, number_data)
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
            "NULL".cell().justify(Justify::Right).bold(true),
            "Min".cell().justify(Justify::Right).bold(true),
            "Max".cell().justify(Justify::Right).bold(true),
            "Mean".cell().justify(Justify::Right).bold(true),
            "StdDev".cell().justify(Justify::Right).bold(true),
        ];
        group_title.append(&mut number_title);

        let table = self
            .output_rows
            .iter()
            .map(|output_row| {
                let mut group_data: Vec<CellStruct> =
                    output_row.group_data.iter().map(|v| v.cell()).collect();
                let mut number_data: Vec<CellStruct> = output_row
                    .number_data
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
            ["count", "null", "min", "max", "mean", "stddev"].join(&delimiter)
        );
        for row in self.output_rows.iter() {
            println!(
                "{}{}{}",
                row.group_data.join(&delimiter),
                delimiter,
                row.number_data.join(&delimiter)
            );
        }
    }
}
