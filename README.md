# Analyze data from stream or file

```
Usage: analyze_data [OPTIONS] --input-delimiter <INPUT_DELIMITER> [MODE] [FILE]

Arguments:
  [MODE]
          What mode to run the program in

          [default: number]

          Possible values:
          - number: Run stats on last column as number and interpret preceding columns as group
          - string: Run stats on last column as string and interpret preceding columns as group
          - csv:    Interpret input as CSV with headers and run stats for all

  [FILE]
          The path to the file to read, use - to read from stdin (must not be a tty)

          [default: -]

Options:
  -d, --input-delimiter <INPUT_DELIMITER>
          input delimiter

  -D, --output-delimiter <OUTPUT_DELIMITER>
          Optional output delimiter, default to human readable table output

  -p, --precision <PRECISION>
          Optional number of decimals to round for output

          [default: 0]

  -z, --zero-as-empty
          Count zeros as empty when parsing numbers

  -h, --help
          Print help (see a summary with '-h')
```
