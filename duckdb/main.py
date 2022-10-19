import os
import time

import duckdb

queries = [
    "select * from '%s' where service = 'frontend'",
    "select * from '%s' where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal'",
    "select * from '%s' where service = 'frontend' and host = 'i-1ec3d9e2506434b2.ec2.internal' and time > '1970-01-01 00:00:00.008000'::timestamp",
]

file = os.getenv("FILE")

for query in queries:
    start = time.process_time_ns()
    duckdb.query(query % file).execute()
    end = time.process_time_ns()
    elapsed = (end - start) / 1000000000
    print(f"{query} - {elapsed}s")
