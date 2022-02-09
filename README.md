# csvsql-rs

Query a CSV file using SQL. The table name is the stem of the CSV file path.

## Usage

### Example

```Shell
$ csvsql -f assets/easy.csv -q "select * from easy"
```

### Help

```
csvsql 0.1.0
Michael P. Moran
Query a CSV file using SQL.

USAGE:
    csvsql [OPTIONS]

OPTIONS:
    -f <file>         Path to CSV file
    -h, --help        Print help information
    -q <query>        SQL query
    -V, --version     Print version information
```
