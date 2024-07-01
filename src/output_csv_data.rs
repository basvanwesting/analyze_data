use crate::output_row::OutputRow;
use crate::CsvStats;
use cli_table::{
    format::{HorizontalLine, Justify, Separator, VerticalLine},
    print_stdout, Cell, CellStruct, Style, Table,
};

pub struct OutputCsvData {
    output_rows: Vec<OutputRow>,
    group_length: usize,
    output_delimiter: Option<char>,
}

impl OutputCsvData {
    pub fn new(
        csv_stats: CsvStats,
        _input_delimiter: char,
        output_delimiter: Option<char>,
        decimals: usize,
    ) -> Self {
        let output_rows: Vec<OutputRow> = csv_stats
            .into_iter()
            .map(|(header, string_stats, number_stats, length_stats)| {
                let stats_data = vec![
                    format!("{}", string_stats.count()),
                    format!("{}", string_stats.cardinality()),
                    format!("{}", string_stats.empty_count()),
                    format!("{}", string_stats.min().unwrap_or("".to_string())),
                    format!("{}", string_stats.max().unwrap_or("".to_string())),
                    format!("{}", number_stats.empty_count()),
                    format!("{}", number_stats.error_count()),
                    format!("{:.*}", decimals, number_stats.min().unwrap_or(0.0),),
                    format!("{:.*}", decimals, number_stats.max().unwrap_or(0.0),),
                    format!("{:.*}", decimals, number_stats.mean()),
                    format!("{:.*}", decimals, number_stats.stddev()),
                    format!("{:.*}", 0, length_stats.min().unwrap_or(0.0),),
                    format!("{:.*}", 0, length_stats.max().unwrap_or(0.0),),
                    format!("{:.*}", decimals, length_stats.mean()),
                    format!("{:.*}", decimals, length_stats.stddev()),
                ];
                OutputRow::new([header].to_vec(), stats_data)
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
            "Cardinality".cell().justify(Justify::Right).bold(true),
            "String Empty".cell().justify(Justify::Right).bold(true),
            "String Min".cell().justify(Justify::Right).bold(true),
            "String Max".cell().justify(Justify::Right).bold(true),
            "Number Empty".cell().justify(Justify::Right).bold(true),
            "Number Error".cell().justify(Justify::Right).bold(true),
            "Number Min".cell().justify(Justify::Right).bold(true),
            "Number Max".cell().justify(Justify::Right).bold(true),
            "Number Mean".cell().justify(Justify::Right).bold(true),
            "Number StdDev".cell().justify(Justify::Right).bold(true),
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
                "cardinality",
                "string_empty",
                "string_min",
                "string_max",
                "number_empty",
                "number_error",
                "number_min",
                "number_max",
                "number_mean",
                "number_stddev",
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
