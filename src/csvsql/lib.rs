use std::error::Error;
use std::fmt;
use std::path::Path;

use rusqlite::{Connection, Result};

#[derive(Debug)]
pub enum CsvSqlError {
    QueryError,
}

impl Error for CsvSqlError {}

impl fmt::Display for CsvSqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CsvSqlError::QueryError => write!(f, "There was a problem with the query."),
        }
    }
}

#[inline]
pub fn query(path: &str, query: &str) -> Result<String, Box<dyn Error>> {
    let path_obj = Path::new(path);
    let path_str = path_obj.to_str().unwrap();
    let table = path_obj.file_stem().unwrap().to_str().unwrap();
    log::debug!("csv path\n{}", path_str);

    // create sqlite database
    log::info!("Creating SQLite database.");
    let db = Connection::open_in_memory()?;
    // load csv sqlite module
    log::info!("Loading CSV SQLite module.");
    rusqlite::vtab::csvtab::load_module(&db)?;
    // create sqlite table
    log::info!("Creating table from CSV file.");
    let schema = format!(
        "
        CREATE VIRTUAL TABLE {}
        USING csv(filename='{}', header=yes)
    ",
        table, path_str
    );
    db.execute_batch(&schema)?;

    // query table
    log::info!("Querying table.");
    let mut stmt = match db.prepare(query) {
        Ok(stmt) => stmt,
        Err(e) => {
            log::error!("There was a problem with the query: {}", e);
            return Err(Box::new(CsvSqlError::QueryError));
        }
    };
    let mut results_stage_1: Vec<Vec<String>> = Vec::new();
    let rows = stmt.query_map([], |row| {
        let mut vals: Vec<String> = Vec::new();
        for idx in 0.. {
            match row.get(idx) {
                Ok(val) => vals.push(val),
                Err(_) => break,
            };
        }
        results_stage_1.push(vals);
        Ok(())
    })?;
    let count = rows.count(); // exhaust the iterable; which will fill results vector with values from each row
    log::debug!("row count\n{}", count);
    // add column names to results vector
    let column_names = stmt.column_names();
    results_stage_1.insert(0, column_names.iter().map(|s| s.to_string()).collect());
    log::debug!("column names\n{:?}", column_names);
    log::debug!("raw results\n{:?}", results_stage_1);

    log::info!("Converting query results to CSV format.");
    let mut results_stage_2: Vec<String> = Vec::new();
    for row in results_stage_1.iter() {
        let row2 = row.join(",");
        results_stage_2.push(row2);
    }
    let results_stage_3 = results_stage_2.join("\n");

    // print csv output
    log::info!("Writing CSV results.");
    let mut rdr = csv::Reader::from_reader(results_stage_3.as_bytes());
    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&column_names)?;
    for res in rdr.records() {
        let rec = res?;
        wtr.write_record(&rec)?;
    }
    let results = String::from_utf8(wtr.into_inner()?)?;

    // drop the table
    log::info!("Dropping table.");
    db.execute_batch(format!("DROP TABLE {}", table).as_str())?;

    log::info!("Done.");

    Ok(results)
}
