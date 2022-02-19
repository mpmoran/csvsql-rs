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

    let results = csvsql::query(file, query)?;
    print!("{}", results);

    Ok(())
}
