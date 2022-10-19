use polars::prelude::*;
use std::time::Instant;

fn main() {
    let path = std::env::var("FILE").unwrap();

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .filter(col("service").eq(lit("frontend")));
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select * from logs where service = 'frontend' - {}",
        elapsed.as_secs_f64()
    );

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .filter(col("service").eq(lit("frontend")))
        .filter(col("host").eq(lit("i-1ec3d9e2506434b2.ec2.internal")));
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select * from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' - {}",
        elapsed.as_secs_f64()
    );

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .filter(col("service").eq(lit("frontend")))
        .filter(col("host").eq(lit("i-1ec3d9e2506434b2.ec2.internal")))
        .filter(col("time").gt(cast(
            lit("1970-01-01 00:00:00.008000"),
            DataType::Datetime(TimeUnit::Nanoseconds, None),
        )));

    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select * from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp - {}",
        elapsed.as_secs_f64()
    );
}
