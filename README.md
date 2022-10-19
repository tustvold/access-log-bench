# Access Log Bench

Benchmarks of querying access log data using various different libraries.

See https://github.com/tustvold/access-log-gen/ for how to generate some example data

Supported Libraries

* [DataFusion](./datafusion)
* [Polars](./polars)
* [DuckDB](./duckdb)

## Disclaimer

As one of the DataFusion maintainers, I am necessarily more familiar with this project than the others. It is therefore
possible that I have missed something when implementing the benchmarks for the other frameworks. Please feel free to 
file an issue if this is the case.

## Results

Benchmarks run on a GCP c2-standard-16, containing a 16-core, Cascade Lake Intel Xeon

Rust code compiled in release mode with `RUSTFLAGS="-C target-cpu=native"`

|                                                                                                                                                                 | DataFusion   | Polars       | DuckDB       |
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------|--------------|--------------|
| select * from logs                                                                                                                                              | 0.070945046s | 0.460665795  | 3.70185782s  |
| select * from logs where service = 'frontend'                                                                                                                   | 0.031640836s | 0.563334824s | 1.478420882s |
| select * from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal'                                                                      | 0.025971204s | 0.39868975s  | 0.093473218s |
| select * from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp                   | 0.012729506s | 0.394838356s | 0.017670706s |
| select request_duration_ns from logs                                                                                                                            | 0.004057146s | 0.010826365s | 0.034813344s |
| select request_duration_ns from logs where service = 'frontend'                                                                                                 | 0.01511411s  | 0.035965889s | 0.064686456s |
| select request_duration_ns from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal'                                                    | 0.009883877s | 0.034294067s | 0.026720655s |
| select request_duration_ns from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp | 0.008914378s | 0.053213211s | 0.027028755s |
| select client_addr from logs                                                                                                                                    | 0.009847708s | 0.028622422s | 0.429092272s |
| select client_addr from logs where service = 'frontend'                                                                                                         | 0.009888685s | 0.035862489s | 0.171640143s |
| select client_addr from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal'                                                            | 0.009451137s | 0.041778316s | 0.050099823s |
| select client_addr from logs where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp         | 0.008866058s | 0.052782936s | 0.026275229s |
