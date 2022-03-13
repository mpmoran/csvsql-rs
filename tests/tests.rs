use rusqlite::Connection;

use csvsqllib as csvsql;

static EASY_CSV_PATH: &str = "assets/easy.csv";
static EASY_DDL_PATH: &str = "assets/easy_ddl.sql";

#[test]
fn csvsql_query_all_results() {
    let query = "select * from easy";
    let expected = "a,b,c\n1,2,3\n";
    let actual = csvsql::query(EASY_CSV_PATH, query, &false, "").unwrap();
    assert_eq!(expected, actual);

    // TODO write test for when use_insert is false
}

#[test]
fn csvsql_query_one_column() {
    let query = "select a from easy";
    let expected: &str = "a\n1\n";
    let actual = csvsql::query(EASY_CSV_PATH, query, &false, "").unwrap();
    assert_eq!(expected, actual);
    // TODO write test for when use_insert is false
}

#[test]
#[should_panic]
fn csvsql_query_invalid_syntax() {
    let query = "select";
    let _ = csvsql::query(EASY_CSV_PATH, query, &false, "").unwrap();
    // TODO write test for when use_insert is false
}

#[test]
#[should_panic]
fn csvsql_query_invalid_column() {
    let query = "select id from easy";
    let _ = csvsql::query(EASY_CSV_PATH, query, &false, "").unwrap();
    // TODO write test for when use_insert is false
}

#[test]
#[should_panic]
fn csvsql_query_invalid_table() {
    let query = "select * from breakfast";
    let _ = csvsql::query(EASY_CSV_PATH, query, &false, "").unwrap();
    // TODO write test for when use_insert is false
}

#[test]
fn csvsql_create_table_with_csv_mod_easy() {
    let table = "easy";
    let db = Connection::open_in_memory().unwrap();
    csvsql::create_table_with_csv_mod(&db, table, EASY_CSV_PATH).unwrap();
}

#[test]
fn csvsql_create_table_with_insert_easy() {
    let db = Connection::open_in_memory().unwrap();
    csvsql::create_table_with_insert(&db, EASY_CSV_PATH, EASY_DDL_PATH).unwrap();
}
