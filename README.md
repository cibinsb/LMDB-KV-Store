# LMDB-KV-Store
![CI/CD](https://github.com/cibinsb/LMDB-KV-Store/actions/workflows/main.yml/badge.svg)

Key-Value Store Implemented with LMDB.
The LMDB Key-Value store is a web service constructed using Axum, 
offering the capability to store values with automatically generated keys.

This REST API provides endpoints to interact with a key/value store using LMDB. 
It allows setting values for keys, retrieving values for keys, retrieving all keys,
and performing other administrative operations.

# Live Demo

https://lmdb-kv-store.shuttleapp.rs/keys

# Install
To install and run the LMDB-KV-Store, follow these steps:
```
docker build -t lmdb-kv-store .

docker run -d -v data:/LMDB-KV-Store/data --env-file=.env.docker -p 8000:8000 lmdb-kv-store 
```
or 

```
docker run --env-file=.env.docker -p 8000:8000 cibinsb/lmdb-kv-store:latest
```

# API Doc

| Route       | POST /value                                                            |
|-------------|------------------------------------------------------------------------|
| Description | Endpoint to set a value in the key/value(key is auto generated) store. |
| Request  Method  | POST                                                                   |                                                              |
| Request Body        | The value to be saved.                                                 |
| Path        | /value                                                                 |
| Response Status Code | 200 OK                                                                 |

| Route       | GET /{key}  |
|-------------|-------------|
| Description | Endpoint to get the value for a given key from the key/value store. |
| Request  Method   |      GET       |
| Path        | /{key}      |
| Parameters  | key (string): The key to retrieve the value for. |
| Request Body        | The value associated with the given key. |
| Response Status Code | 200 OK      |

| Route       | POST /{key} |
|-------------|-------------|
| Description | Endpoint to set a value for a given key in the key/value store. |
| Request  Method   |     POST        |
| Path        | /{key}      |
| Parameters  | key (string): The key to set the value for. |
| Request Body        | The new value to be associated with the key. |
| Response Status Code | 200 OK      |

| Route       | GET /keys                                                 |
|-------------|-----------------------------------------------------------|
| Description | Endpoint to retrieve all the keys in the key/value store. |
| Request   Method  | GET                                                       |
| Path        | /keys                                                     |
| Response  body  | JSON, A list of all the keys in the key/value store.                                                       |
| Status Code | 200 OK                                                    |

### Admin Endpoint Details


| HTTP Method | Endpoint          | Description                               |
|-------------|-------------------|-------------------------------------------|
| GET         | /admin/keys/count | Retrieve the count of all keys             |
| DELETE      | /admin/keys       | Delete all keys                            |
| DELETE      | /admin/key/:key   | Delete a specific key                      |



#### Retrieve the count of all keys

- HTTP Method: GET
- Endpoint: /admin/keys/count
- Description: Retrieves the count of all keys.
- Authentication: Requires bearer authentication with the `secret-token` header.

#### Delete all keys

- HTTP Method: DELETE
- Endpoint: /admin/keys
- Description: Deletes all keys.
- Authentication: Requires bearer authentication with the `secret-token` header.

#### Delete a specific key

- HTTP Method: DELETE
- Endpoint: /admin/key/:key
- Description: Deletes a specific key identified by the provided `:key` parameter.
- Path Parameters:
  - `:key`: The key to be deleted.
- Authentication: Requires bearer authentication with the `secret-token` header.

Please note that all endpoints in this API require bearer authentication with the "secret-token" header.

Make sure to replace "secret-token" with the actual authentication token when making requests to the API.

# Test

```
export DATABASE_NAME=test.mdb
export SECRET_TOKEN=secret-token
cargo test

```
