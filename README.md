# MemDB
================


[![Rust Version](https://img.shields.io/badge/rust-1.66+-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/workflow/status/oabraham1/memdb/CI)](https://github.com/oabraham1/memdb/actions)


MemDB: An In-Memory Caching Key-Value Store
-----------------------------------------------


### Overview

MemDB is an attempt to create a simple in-memory caching key-value store using Rust. The primary goals of this project are:


*   To gain a deeper understanding of how caching key-value stores work internally.
*   To learn the Rust programming language.


### Features

*   **In-Memory Storage**: Fast and efficient storage for caching purposes.
*   **Key-Value Store**: Store and retrieve data using unique keys.
*   **Simple TCP Server**: Accepts connections and handles basic CRUD operations.


### Architecture


```mermaid
graph TD;
    TCP Server --> Cache Store;
    Cache Store --> Error Handling;
    Cache Store[(HashMap)];
    Error Handling[(Logging & Error Codes)];
```
```mermaid
graph TD;
    TCP Server-->Cache Store;
    Cache Store-->Error Handling;
    Error Handling-->Storage Engine;
    Strorage Engine-->Indexing;
    Indexing-->Node Replication;
```
