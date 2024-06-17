Grouped number stats on stream (count, min, max, mean, stddev). \
Takes the last column of the provided data as the number value to analyze. \
All preceding columns are interpreted as grouping data

```
Usage: group_stats [OPTIONS] --input-delimiter <INPUT_DELIMITER> [FILE]

Arguments:
  [FILE]  The path to the file to read, use - to read from stdin (must not be a tty) [default: -]

Options:
  -d, --input-delimiter <INPUT_DELIMITER>
          input delimiter
  -D, --output-delimiter <OUTPUT_DELIMITER>
          Optional output delimiter, default to human readable table output
  -z, --zero-as-null
          Count zeros as null, in addition to always counting non-numbers as null
  -r, --decimals <DECIMALS>
          Optional number of decimals to round for output [default: 0]
  -h, --help
          Print help
```
