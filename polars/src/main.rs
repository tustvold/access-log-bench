use polars::prelude::*;
use std::time::Instant;

fn main() {
    let path = std::env::var("FILE").unwrap();

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default()).unwrap();
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!("select * from logs - {}", elapsed.as_secs_f64());

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

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .select(&[col("request_duration_ns")]);
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select request_duration_ns from logs - {}",
        elapsed.as_secs_f64()
    );

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .filter(col("service").eq(lit("frontend")))
        .select(&[col("request_duration_ns")]);
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select request_duration_ns from logs where service = 'frontend' - {}",
        elapsed.as_secs_f64()
    );

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .filter(col("service").eq(lit("frontend")))
        .filter(col("host").eq(lit("i-1ec3d9e2506434b2.ec2.internal")))
        .select(&[col("request_duration_ns")]);
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select request_duration_ns from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' - {}",
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
        )))
        .select(&[col("request_duration_ns")]);

    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select request_duration_ns from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp - {}",
        elapsed.as_secs_f64()
    );

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .select(&[col("client_addr")]);
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!("select client_addr from logs - {}", elapsed.as_secs_f64());

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .filter(col("service").eq(lit("frontend")))
        .select(&[col("client_addr")]);
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select client_addr from logs where service = 'frontend' - {}",
        elapsed.as_secs_f64()
    );

    let start = Instant::now();
    let lf1 = LazyFrame::scan_parquet(&path, Default::default())
        .unwrap()
        .filter(col("service").eq(lit("frontend")))
        .filter(col("host").eq(lit("i-1ec3d9e2506434b2.ec2.internal")))
        .select(&[col("client_addr")]);
    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select client_addr from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' - {}",
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
        )))
        .select(&[col("client_addr")]);

    lf1.collect().unwrap();
    let elapsed = start.elapsed();
    println!(
        "select client_addr from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp - {}",
        elapsed.as_secs_f64()
    );
}
