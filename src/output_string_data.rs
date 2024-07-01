use crate::output_row::OutputRow;
use crate::GroupStringStats;
use cli_table::{
    format::{HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout, Cell, CellStruct, Style, Table,
};
use itertools::Itertools;

pub struct OutputStringData {
    output_rows: Vec<OutputRow>,
    group_length: usize,
    output_delimiter: Option<char>,
}

impl OutputStringData {
    pub fn new(
        group_stats: GroupStringStats,
        input_delimiter: char,
        output_delimiter: Option<char>,
        decimals: usize,
        cardinality_cap: Option<usize>,
    ) -> Self {
        let output_rows: Vec<OutputRow> = group_stats
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
            .map(|(group, (value_stats, length_stats))| {
                let group_data: Vec<String> = group
                    .split(input_delimiter)
                    .map(|v| v.to_string())
                    .collect();
                let stats_data = vec![
                    format!("{}", value_stats.count()),
                    format!("{}", value_stats.null_count()),
                    format!("{}", value_stats.min().unwrap_or("".to_string())),
                    format!("{}", value_stats.max().unwrap_or("".to_string())),
                    if let Some(cap) = cardinality_cap {
                        if cap == 0 {
                            format!("disabled")
                        } else if value_stats.is_cardinality_capped() {
                            format!("{}+", cap)
                        } else {
                            format!("{}", value_stats.cardinality())
                        }
                    } else {
                        format!("{}", value_stats.cardinality())
                    },
                    format!("{:.*}", 0, length_stats.min().unwrap_or(0.0),),
                    format!("{:.*}", 0, length_stats.max().unwrap_or(0.0),),
                    format!("{:.*}", decimals, length_stats.mean()),
                    format!("{:.*}", decimals, length_stats.stddev()),
                ];
                OutputRow::new(group_data, stats_data)
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
            "Cardinality".cell().justify(Justify::Right).bold(true),
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
                "null",
                "min",
                "max",
                "cardinality",
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
