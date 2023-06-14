# LMDB-KV-Store
Key-Value Store Implemented with LMDB.
The LMDB Key-Value store is a web service constructed using Axum, 
offering the capability to store values with automatically generated keys.

# Install
To install and run the LMDB-KV-Store, follow these steps:
```
docker build -t lmdb-kv-store .

docker run -d -V data:/LMDB-KV-Store/data -p 8000:8000 lmdb-kv-store 
```

# Test

TODO
