# csvsql-rs

[![master](https://github.com/mpmoran/csvsql-rs/actions/workflows/master.yml/badge.svg)](https://github.com/mpmoran/csvsql-rs/actions/workflows/master.yml)
[![test](https://github.com/mpmoran/csvsql-rs/actions/workflows/test.yml/badge.svg)](https://github.com/mpmoran/csvsql-rs/actions/workflows/test.yml)

Query a CSV file using SQL. The table name is the stem of the CSV file path. The application writes results to standard output by default.

Supported platforms are GNU/Linux, macOS, and Windows.

Inspired by [csvkit](https://github.com/wireservice/csvkit).

## Installation

Installation is a manual process.

### GNU/Linux

```console
mkdir csvsql-rs
cd csvsql-rs
curl -o csvsql-rs.zip -L https://github.com/mpmoran/csvsql-rs/releases/download/0.1.0/csvsql-rs-binary-linux-gnu-x86_64.zip
unzip csvsql-rs.zip
chmod +x csvsql
./csvsql --help
```

### macOS

```console
curl -o csvsql-rs.zip -L https://github.com/mpmoran/csvsql-rs/releases/download/0.1.0/csvsql-rs-binary-apple-darwin-x86_64.zip
unzip csvsql-rs.zip
chmod +x csvsql
```sh
curl -o csvsql-rs.zip -L https://github.com/mpmoran/csvsql-rs/releases/download/0.1.0/csvsql-rs-binary-linux-gnu-x86_64.zip
unzip csvsql-rs.zip
chmod +x csvsql
sudo cp csvsql /usr/local/bin
csvsql --help
```

## Usage

### Example

#### Input

`assets/easy.csv`

```CSV
a,b,c
1,2,3

```

#### Run

```console
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
$ csvsql --help
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

### Uninstallation

#### GNU/Linux and macOS

```sh
sudo rm /usr/local/bin/csvsql
```
