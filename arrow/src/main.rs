//! Test running the following query manually
//!
//! ```
//! "select client_addr from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp"
//! ```

use bytes::Bytes;
use futures::future::BoxFuture;
use futures::{FutureExt, StreamExt};
use parquet::arrow::arrow_reader::ParquetRecordBatchReader;
use parquet::arrow::async_reader::AsyncFileReader;
use parquet::arrow::ParquetRecordBatchStreamBuilder;
use parquet::errors::Result;
use parquet::file::footer;
use parquet::file::metadata::ParquetMetaData;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let path = std::env::var("FILE").unwrap();

    let queries: &[(&'static str, &dyn Fn(&str))] = &[
        ("select_all_sync", &select_all_sync),
        ("select_all_async", &select_all_async),
    ];

    for (query, query_fn) in queries {
        let start = Instant::now();
        query_fn(&path);
        let elapsed = start.elapsed();
        println!("{} - {}s", query, elapsed.as_secs_f64());
    }
}

fn select_all_sync(path: &str) {
    let mut file = File::open(path).unwrap();
    let mut out = vec![];
    file.read_to_end(&mut out).unwrap();
    let bytes: Bytes = out.into();

    let mut reader = ParquetRecordBatchReader::try_new(bytes, 1024).unwrap();
    for r in reader.next() {
        r.unwrap();
    }
}

fn select_all_async(path: &str) {
    let file = SyncReader(File::open(path).unwrap());
    futures::executor::block_on(async move {
        let mut stream = ParquetRecordBatchStreamBuilder::new(file)
            .await
            .unwrap()
            .build()
            .unwrap();
        for r in stream.next().await {
            r.unwrap();
        }
    })
}

struct SyncReader(File);

impl SyncReader {
    fn get_bytes_sync(&mut self, range: Range<usize>) -> Result<Bytes> {
        let len = range.end - range.start;
        self.0.seek(SeekFrom::Start(range.start as u64))?;
        let mut out = Vec::with_capacity(len);
        (&mut self.0).take(len as u64).read_to_end(&mut out)?;
        Ok(out.into())
    }

    fn get_metadata_sync(&mut self) -> Result<Arc<ParquetMetaData>> {
        Ok(Arc::new(footer::parse_metadata(&self.0)?))
    }
}

impl AsyncFileReader for SyncReader {
    fn get_bytes(&mut self, range: Range<usize>) -> BoxFuture<'_, Result<Bytes>> {
        futures::future::ready(self.get_bytes_sync(range)).boxed()
    }

    fn get_metadata(&mut self) -> BoxFuture<'_, Result<Arc<ParquetMetaData>>> {
        futures::future::ready(self.get_metadata_sync()).boxed()
    }
}
