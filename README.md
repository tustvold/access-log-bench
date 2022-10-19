# Access Log Bench

Benchmarks of querying access log data using various different libraries.

See https://github.com/tustvold/access-log-gen/ for how to generate some example data

Supported Libraries

* [DataFusion](./datafusion)
* [Polars](./polars)
* [DuckDB](./duckdb)

## Disclaimer

As one of the DataFusion maintainers, I am necessarily more familiar with this project than the others. It is therefore
possible that I have missed something when implementing the benchmarks for the other frameworks.

## Results

Benchmarks run on a GCP c2-standard-16, containing a 16-core, Cascade Lake Intel Xeon

Rust code compiled in release mode with `RUSTFLAGS="-C target-cpu=native"`

|                                                                                                                                               | DataFusion   | Polars       | DuckDB       |
|-----------------------------------------------------------------------------------------------------------------------------------------------|--------------|--------------|--------------|
| select * from logs where service = 'frontend'                                                                                                 | 0.418068068s | 0.563334824s | 1.478420882s |
| select * from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal'                                                    | 0.040262937s | 0.39868975s  | 0.093473218s |
| select * from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp | 0.010112098s | 0.394838356s | 0.017670706s |
