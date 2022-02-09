use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use csvsqllib as csvsql;

pub fn csvsql_query_replication(c: &mut Criterion) {
    let mut group = c.benchmark_group("csvsql_query");
    let duration = Duration::from_secs(7);
    group.measurement_time(duration);
    group.bench_function("csvsql_query_replication", |b| {
        b.iter(|| {
            csvsql::query(
                black_box("assets/easy.csv"),
                black_box("select * from easy"),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, csvsql_query_replication);
criterion_main!(benches);
