# csvsql-rs

[![test](https://github.com/mpmoran/csvsql-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/mpmoran/csvsql-rs/actions/workflows/rust.yml)
[![1-create-aarch64-build](https://github.com/mpmoran/csvsql-rs/actions/workflows/rust.yml/badge.svg?branch=1-create-aarch64-build)](https://github.com/mpmoran/csvsql-rs/actions/workflows/rust.yml)

Query a CSV file using SQL. The table name is the stem of the CSV file path. The application writes results to standard output by default.

## Usage

### Example

#### Input

`assets/easy.csv`

```CSV
a,b,c
1,2,3

```

#### Run

```sh
$ csvsql -f assets/easy.csv -q "select * from easy"
a,b,c
1,2,3

```

#### Run, saving results to a file

```sh
csvsql -f assets/easy.csv -q "select * from easy" > out.csv
```

`out.csv`

```CSV
a,b,c
1,2,3

```

### Help

```console
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
