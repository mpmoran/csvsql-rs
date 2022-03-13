use std::error::Error;

use clap::{Command, Arg};

use csvsqllib as csvsql;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let app = Command::new("csvsql")
        .version("0.1.2")
        .author("Michael P. Moran")
        .about("Query a CSV file using SQL.")
        .arg(
            Arg::new("file")
                .short('f')
                .takes_value(true)
                .help("Path to CSV file"),
        )
        .arg(
            Arg::new("query")
                .short('q')
                .takes_value(true)
                .help("SQL query"),
        )
        .arg(
            Arg::new("insert")
                .short('i')
                .takes_value(false)
                .help("Create table with insert statements instead of SQLite CSV module")
                .required(false),
        )
        .arg(
            Arg::new("ddl-file")
                .short('d')
                .takes_value(true)
                .help("Path to file containing DDL for creating table; required only if -i is provided.")
                .required(false),
        );

    let mut app_clone = app.clone();
    let matches = app.get_matches();
    let file = match matches.value_of("file") {
        Some(file) => file,
        None => {
            log::error!("You didn't provide a CSV path.");
            app_clone.print_help()?;
            return Ok(());
        }
    };
    let query = match matches.value_of("query") {
        Some(query) => query,
        None => {
            log::error!("You didn't provide a SQL query.");
            app_clone.print_help()?;
            return Ok(());
        }
    };
    let use_insert = matches.is_present("insert");
    let ddl = match matches.value_of("ddl-file") {
        Some(file) => file,
        None => {
            if use_insert == true {
                log::error!("You specified the -i flag but did not provide a value for --ddl-file.");
                app_clone.print_help()?;
                return Ok(());
            }
            else {
                ""
            }
        }
    };

    let results = csvsql::query(file, query, &use_insert, ddl)?;
    print!("{}", results);

    Ok(())
}
