use datafusion::config::{
    OPT_PARQUET_ENABLE_PAGE_INDEX, OPT_PARQUET_PUSHDOWN_FILTERS, OPT_PARQUET_REORDER_FILTERS,
};
use datafusion::prelude::{SessionConfig, SessionContext};
use std::time::Instant;

const QUERIES: &[&str] = &[
    "select * from logs",
    "select * from logs where service = 'frontend'",
    "select * from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal'",
    "select * from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp",
    "select request_duration_ns from logs",
    "select request_duration_ns from logs where service = 'frontend'",
    "select request_duration_ns from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal'",
    "select request_duration_ns from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp",
    "select client_addr from logs",
    "select client_addr from logs where service = 'frontend'",
    "select client_addr from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal'",
    "select client_addr from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp",
];

#[tokio::main]
async fn main() {
    let path = std::env::var("FILE").unwrap();

    let config = SessionConfig::default()
        .with_collect_statistics(true)
        .with_batch_size(1024 * 8)
        .set_bool(OPT_PARQUET_ENABLE_PAGE_INDEX, true)
        .set_bool(OPT_PARQUET_PUSHDOWN_FILTERS, true)
        .set_bool(OPT_PARQUET_REORDER_FILTERS, true);

    let ctx = SessionContext::with_config(config);
    ctx.register_parquet("logs", &path, Default::default())
        .await
        .unwrap();

    for query in QUERIES {
        let start = Instant::now();
        ctx.sql(query).await.unwrap().collect().await.unwrap();

        let elapsed = start.elapsed();
        println!("{} - {}s", query, elapsed.as_secs_f64());
    }
}
