use std::error::Error;
use std::fs;
use std::fmt;
use std::fs::OpenOptions;
use std::path::Path;

use rusqlite::{Connection, Result};

#[derive(Debug)]
pub enum CsvSqlError {
    QueryError,
    CreateTableError,
}

impl Error for CsvSqlError {}

impl fmt::Display for CsvSqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CsvSqlError::QueryError => write!(f, "There was a problem with the query."),
            CsvSqlError::CreateTableError => write!(f, "There was a problem creating a table from the provided CSV file.")
        }
    }
}

#[inline]
pub fn query(path: &str, query: &str, use_insert: &bool, ddl_path: &str) -> Result<String, Box<dyn Error>> {
    let path_obj = Path::new(path);
    let path_str = path_obj.to_str().unwrap();
    let table = path_obj.file_stem().unwrap().to_str().unwrap();
    log::debug!("csv path\n{}", path_str);

    // create sqlite database
    log::info!("Creating SQLite database.");
    let db = Connection::open_in_memory()?;

    if use_insert == &true {
        create_table_with_insert(&db, &path_str, &ddl_path)?;
    } else {
        create_table_with_csv_mod(&db, &table, &path_str)?;
    }

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
    let count = rows.count(); // exhaust the iterable, which will fill results vector with values from each row
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

pub fn create_table_with_csv_mod(db: &Connection, table: &str, path: &str) -> Result<(), CsvSqlError> {
    // load csv sqlite module
    log::info!("Loading CSV SQLite module.");
    match rusqlite::vtab::csvtab::load_module(&db) {
        Ok(()) => (),
        Err(e) => {
            log::error!("Failed to load CSV SQLite module because of this error: {}", e);
            return Err(CsvSqlError::CreateTableError);
        }
    }

    // create sqlite table
    log::info!("Creating table from CSV file.");
    let ct_stmt = format!(
        "
        CREATE VIRTUAL TABLE {}
        USING csv(filename='{}', header=yes)
    ",
        table, path
    );
    match db.execute_batch(&ct_stmt) {
        Ok(()) => Ok(()),
        Err(e) => {
            log::error!("Failed to create table because of this error: {}", e);
            return Err(CsvSqlError::CreateTableError);
        }
    }
}

pub fn create_table_with_insert(db: &Connection, path: &str, ddl_path: &str) -> Result<(), CsvSqlError> {
    log::info!("Opening and reading DDL file.");
    let ddl= match fs::read_to_string(ddl_path) {
        Ok(contents) => contents,
        Err(e) => {
            log::error!("Failed to open and read DDL file because of this error: {}", e);
            return Err(CsvSqlError::CreateTableError)
        }
    };
    log::debug!("table ddl\n{:?}", ddl);

    log::info!("Opening CSV file at {}.", path);
    let csv_file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(e) => {
            log::error!("Failed to open CSV file because of this error: {}", e);
            return Err(CsvSqlError::CreateTableError)
        }
    };
    log::info!("Reading CSV file.");
    let mut records: Vec<csv::StringRecord> = Vec::new();
    let mut rdr = csv::Reader::from_reader(&csv_file);
    for result in rdr.records() {
        let record = result.unwrap();
        records.push(record);
    }
    log::debug!("csv records\n{:?}", records);

    // log::info!("Composing table schema.");
    // let header = &records[0];
    // let mut schema = String::new();
    // for field in header.iter() {
    //     schema.push_str(format!("{} TEXT,\n", field).as_str());
    // }
    // let schema = schema.trim_end_matches(",\n");
    // log::debug!("schema\n{:?}", schema);


    // log::info!("Creating table from CSV file.");
    // let ct_stmt = format!("CREATE TABLE {} ({})", table, schema
    // );
    // match db.execute_batch(&ct_stmt) {
    //     Ok(()) => Ok(()),
    //     Err(e) => {
    //         log::error!("Failed to create table because of this error: {}", e);
    //         return Err(CsvSqlError::CreateTableError);
    //     }
    // }

    Ok(())

    // read csv

        // for field in record.iter() {

        // }
    // close file
}