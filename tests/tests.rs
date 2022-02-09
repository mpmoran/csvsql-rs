use csvsqllib as csvsql;

static CSV_PATH_EASY: &str = "assets/easy.csv";

#[test]
fn csvsql_query_all_results() {
    let query = "select * from easy";
    let expected = "a,b,c\n1,2,3\n";
    let actual = csvsql::query(CSV_PATH_EASY, query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn csvsql_query_one_column() {
    let query = "select a from easy";
    let expected: &str = "a\n1\n";
    let actual = csvsql::query(CSV_PATH_EASY, query).unwrap();
    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn csvsql_query_invalid_syntax() {
    let query = "select";
    let _ = csvsql::query(CSV_PATH_EASY, query).unwrap();
}

#[test]
#[should_panic]
fn csvsql_query_invalid_column() {
    let query = "select id from easy";
    let _ = csvsql::query(CSV_PATH_EASY, query).unwrap();
}

#[test]
#[should_panic]
fn csvsql_query_invalid_table() {
    let query = "select * from breakfast";
    let _ = csvsql::query(CSV_PATH_EASY, query).unwrap();
}
