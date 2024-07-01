Grouped number or string stats on stream (count, min, max, mean, stddev). \ 
Takes the last column of the provided data as the number (default) or string value to analyze. \
All preceding columns are interpreted as grouping data.

```
Usage: group_stats [OPTIONS] --input-delimiter <INPUT_DELIMITER> [FILE]

Arguments:
  [FILE]  The path to the file to read, use - to read from stdin (must not be a tty) [default: -]

Options:
  -d, --input-delimiter <INPUT_DELIMITER>
          input delimiter
  -D, --output-delimiter <OUTPUT_DELIMITER>
          Optional output delimiter, default to human readable table output
  -r, --decimals <DECIMALS>
          Optional number of decimals to round for output [default: 0]
  -n, --null
          Count zeros as null in number mode. Count empty strings as null in string mode
  -s, --strings
          Interpret as strings instead of numbers (default), returns stats about length and value
  -h, --help
          Print help
```
